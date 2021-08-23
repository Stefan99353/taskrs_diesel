import {Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {Observable} from 'rxjs';
import {environment} from '../../../environments/environment';
import {shareReplay, tap} from 'rxjs/operators';
import {decodeJwt} from '../utils';
import {DateTime} from 'luxon';
import {Router} from '@angular/router';

export interface UserTokens {
    accessToken: string,
    refreshToken: string,
}

@Injectable({
    providedIn: 'root',
})
export class AuthService {
    private baseUrl = environment.baseUrl + 'auth';
    private refreshTimer = 0;

    constructor(
        private httpClient: HttpClient,
        private router: Router,
    ) {
        this.refreshToken().subscribe();
    }

    login(email: string, password: string): Observable<UserTokens> {
        return this.httpClient
            .post<UserTokens>(this.baseUrl + '/login', {email, password})
            .pipe(
                tap(tokens => {
                    this.setSession(tokens);
                    this.startRefreshTimer();
                }),
                shareReplay(),
            );
    }

    logout(): Observable<void> {
        const refreshToken: string | null = localStorage.getItem('refreshToken');
        const headers = new HttpHeaders({
            'Content-Type': 'application/json; charset=utf-8',
        });

        return this.httpClient
            .post<void>(this.baseUrl + '/logout', JSON.stringify(refreshToken), {headers})
            .pipe(
                tap(() => {
                    this.removeSession();
                    this.stopRefreshTimer();
                }, () => {
                    this.removeSession();
                    this.stopRefreshTimer();
                }),
            );
    }

    refreshToken(): Observable<string> {
        const refreshToken: string | null = localStorage.getItem('refreshToken');
        const headers = new HttpHeaders({
            'Content-Type': 'application/json; charset=utf-8',
        });

        if (refreshToken === null) {
            this.router.navigateByUrl('/login');
        }

        return this.httpClient
            .post<string>(this.baseUrl + '/token', JSON.stringify(refreshToken), {headers})
            .pipe(
                tap(token => {
                    const expiresAt = decodeJwt(token).exp;
                    localStorage.setItem('accessToken', token);
                    localStorage.setItem('accessTokenExp', JSON.stringify(expiresAt));
                    this.startRefreshTimer();
                }),
            );
    }

    private setSession(tokens: UserTokens): void {
        const expiresAt = decodeJwt(tokens.accessToken).exp;

        localStorage.setItem('accessToken', tokens.accessToken);
        localStorage.setItem('refreshToken', tokens.refreshToken);
        localStorage.setItem('accessTokenExp', JSON.stringify(expiresAt));
    }

    private removeSession(): void {
        localStorage.removeItem('accessToken');
        localStorage.removeItem('refreshToken');
        localStorage.removeItem('accessTokenExp');
    }

    private startRefreshTimer(): void {
        const expiration = this.getAccessTokenExpiration();

        if (expiration) {
            let timeout = expiration.diff(DateTime.now(), 'milliseconds').milliseconds - environment.refreshTimeBuffer * 1000;
            this.refreshTimer = setTimeout(() => this.refreshToken().subscribe(), timeout);
        }
    }

    private stopRefreshTimer(): void {
        clearTimeout(this.refreshTimer);
    }

    isLoggedIn(): boolean {
        const expiration: DateTime | null = this.getAccessTokenExpiration();
        if (expiration) {
            return expiration >= DateTime.now();
        }

        return false;
    }

    isLoggedOut(): boolean {
        const expiration: DateTime | null = this.getAccessTokenExpiration();
        if (expiration) {
            return expiration < DateTime.now();
        }

        return false;
    }

    getAccessTokenExpiration(): DateTime | null {
        const expiration = localStorage.getItem('accessTokenExp');
        if (expiration) {
            const expiresAt: number = JSON.parse(expiration);
            return DateTime.fromSeconds(expiresAt);
        } else {
            return null;
        }
    }
}

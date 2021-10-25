import {Injectable} from '@angular/core';
import {
    HttpErrorResponse,
    HttpEvent,
    HttpHandler,
    HttpInterceptor,
    HttpRequest,
    HttpStatusCode,
} from '@angular/common/http';
import {Observable, throwError} from 'rxjs';
import {catchError} from 'rxjs/operators';
import {Router} from '@angular/router';

@Injectable()
export class AuthInterceptor implements HttpInterceptor {

    constructor(private router: Router) {
    }

    intercept(
        request: HttpRequest<any>,
        next: HttpHandler,
    ): Observable<HttpEvent<any>> {
        // Add bearer token
        let accessToken: string | null = localStorage.getItem('accessToken');
        if (accessToken) {
            request = request.clone({
                headers: request.headers.set('Authorization', 'Bearer ' + accessToken),
            });
        }

        return next.handle(request).pipe(
            catchError(
                (
                    httpErrorResponse: HttpErrorResponse,
                    _: Observable<HttpEvent<any>>
                ) => {
                    if (httpErrorResponse.status === HttpStatusCode.Unauthorized) {
                        this.router.navigateByUrl('/login');
                    }

                    return throwError(httpErrorResponse);
                }
            )
        );
    }
}

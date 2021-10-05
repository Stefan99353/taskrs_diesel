import { Injectable } from '@angular/core';
import {environment} from '../../../environments/environment';
import {HttpClient, HttpParams} from '@angular/common/http';
import {createRequestFilterParams, RequestFilter} from '../models/request-filter';
import {Observable} from 'rxjs';
import {PaginationPage} from '../models/pagination-page';
import {Permission} from '../models/permission';
import {User} from '../models/user';
import {Category} from '../models/category';

@Injectable({
  providedIn: 'root'
})
export class UsersService {
    private baseUrl = environment.baseUrl + 'users';

    constructor(
        private httpClient: HttpClient,
    ) {
    }

    allUsers(filter: RequestFilter | null): Observable<PaginationPage<User>> {
        const params = createRequestFilterParams(filter);
        return this.httpClient.get<PaginationPage<User>>(this.baseUrl, {params});
    }

    createUser(user: User): Observable<User> {
        return this.httpClient.post<User>(this.baseUrl, user);
    }

    deleteUser(id: number): Observable<void> {
        const params = new HttpParams()
            .set('id', id);

        return this.httpClient.delete<void>(this.baseUrl, {params});
    }

    updateUser(user: User): Observable<User> {
        return this.httpClient.put<User>(this.baseUrl, user);
    }
}

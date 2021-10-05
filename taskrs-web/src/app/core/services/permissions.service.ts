import {Injectable} from '@angular/core';
import {environment} from '../../../environments/environment';
import {HttpClient} from '@angular/common/http';
import {createRequestFilterParams, RequestFilter} from '../models/request-filter';
import {Observable} from 'rxjs';
import {PaginationPage} from '../models/pagination-page';
import {Permission} from '../models/permission';

@Injectable({
    providedIn: 'root',
})
export class PermissionsService {
    private baseUrl = environment.baseUrl + 'permissions';

    constructor(
        private httpClient: HttpClient,
    ) {
    }

    allPermissions(filter: RequestFilter | null): Observable<PaginationPage<Permission>> {
        const params = createRequestFilterParams(filter);
        return this.httpClient.get<PaginationPage<Permission>>(this.baseUrl, {params});
    }

    grantPermissions(userId: number, permissionIds: number[]): Observable<void> {
        return this.httpClient.post<void>(this.baseUrl + '/grant', {
            userId: userId,
            permissionIds: permissionIds,
        });
    }

    revokePermissions(userId: number, permissionIds: number[]): Observable<void> {
        return this.httpClient.post<void>(this.baseUrl + '/revoke', {
            userId: userId,
            permissionIds: permissionIds,
        });
    }

    setUserPermissions(userId: number, permissionIds: number[]): Observable<void> {
        return this.httpClient.post<void>(this.baseUrl + '/set', {
            userId: userId,
            permissionIds: permissionIds,
        });
    }
}

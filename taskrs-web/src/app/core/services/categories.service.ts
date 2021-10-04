import {Injectable} from '@angular/core';
import {HttpClient, HttpParams} from '@angular/common/http';
import {environment} from '../../../environments/environment';
import {Observable} from 'rxjs';
import {PaginationPage} from '../models/pagination-page';
import {Category} from '../models/category';
import {createRequestFilterParams, RequestFilter} from '../models/request-filter';

@Injectable({
    providedIn: 'root',
})
export class CategoriesService {
    private baseUrl = environment.baseUrl + 'categories';

    constructor(
        private httpClient: HttpClient,
    ) {
    }

    allCategories(filter: RequestFilter | null): Observable<PaginationPage<Category>> {
        const params = createRequestFilterParams(filter);
        return this.httpClient.get<PaginationPage<Category>>(this.baseUrl, {params});
    }

    subCategories(id: number | null): Observable<Category[]> {
        let params = new HttpParams();
        if (id !== null) {
            params = params.set('id', id);
        }
        return this.httpClient.get<Category[]>(this.baseUrl + '/sub', {params});
    }

    createCategory(category: Category): Observable<Category> {
        return this.httpClient.post<Category>(this.baseUrl, category);
    }

    deleteCategory(id: number, cascade: boolean | null): Observable<void> {
        let params = new HttpParams()
            .set('id', id);

        if (cascade !== null) {
            params = params.set('cascade', cascade);
        }

        return this.httpClient.delete<void>(this.baseUrl, {params});
    }

    updateCategory(category: Category): Observable<Category> {
        return this.httpClient.put<Category>(this.baseUrl, category);
    }
}

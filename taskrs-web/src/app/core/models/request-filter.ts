import {HttpParams} from '@angular/common/http';

export interface RequestFilter {
    query: string | null,
    orderBy: string | null,
    order: 'ascending' | 'descending' | null,
    page: number | null,
    limit: number | null,
}

export function createRequestFilterParams(filter: RequestFilter | null): HttpParams {
    let params = new HttpParams();

    if (filter === null) {
        return params;
    }

    if (filter.query !== null) {
        params = params.set('query', filter.query);
    }
    if (filter.orderBy !== null) {
        params = params.set('orderBy', filter.orderBy);
    }
    if (filter.page !== null) {
        params = params.set('page', filter.page);
    }
    if (filter.page !== null) {
        params = params.set('page', filter.page);
    }
    if (filter.limit !== null) {
        params = params.set('limit', filter.limit);
    }

    return params;
}

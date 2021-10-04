export interface PaginationPage<T> {
    page: number | null,
    pageCount: number | null,
    pageSize: number | null,
    totalCount: number | null,
    items: T[],
}

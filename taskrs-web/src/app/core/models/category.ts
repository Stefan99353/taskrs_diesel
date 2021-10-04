export interface Category {
    id: number,
    name: string,
    parentCategoryId: number | null,
    updatedAt: string | null,
    createdAt: string | null,
}

export enum CategoryColumns {
    Id = 'id',
    Name = 'name',
    ParentCategoryId = 'parentCategoryId',
    UpdatedAt = 'updatedAt',
    CreatedAt = 'createdAt',
}

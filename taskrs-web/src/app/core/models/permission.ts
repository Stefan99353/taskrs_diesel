export interface Permission {
    id: number,
    name: string,
    group: string,
    description: string | null,
    updatedAt: string | null,
    createdAt: string | null,
}

export enum PermissionColumns {
    Id = 'id',
    Name = 'name',
    Group = 'group',
    Description = 'description',
    UpdatedAt = 'updatedAt',
    CreatedAt = 'createdAt',
}

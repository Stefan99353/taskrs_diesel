export interface User {
    id: number,
    email: string,
    password: string,
    firstName: string | null,
    lastName: string | null,
    activated: boolean,
    updatedAt: string | null,
    createdAt: string | null,
}

export enum UserColumns {
    Id = 'id',
    Email = 'email',
    Password = 'password',
    FirstName = 'firstName',
    LastName = 'lastName',
    Activated = 'activated',
    UpdatedAt = 'updatedAt',
    CreatedAt = 'createdAt',
}

# API Notes

# Goals

## Manage Puzzles

```
Puzzle {
    id: string;
    name: string;
    created_at: Date;
    updated_at: Date;
    released_at: Date;
    words: string[];
    letters: string[];
    middle: string;
    language: string;
}
```

- Get Puzzle
- Put Puzzle
- Post Puzzle
- Delete Puzzle

- Get Puzzles

## Manage Users

```
User {
    id: string;
    name: string;
    email: string;
    admin: boolean;
    created_at: Date;
    updated_at: Date;
}
```

- Get User
- Put User
- Post User
- Delete User

- Get Users

## Manage Saves

```
Save {
    id: string;
    user_id: string;
    words: string[];
    created_at: Date;
    updated_at: Date;
}
```

- Get Save
- Put Save
- Post Save
- Delete Save

- Get Saves

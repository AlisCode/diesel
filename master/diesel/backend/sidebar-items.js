initSidebarItems({"trait":[["Backend","A database backend"],["BinaryRawValue","A trait indicating that the provided raw value uses a binary representation internally"],["HasRawValue","The raw representation of a database value given to `FromSql`."],["SupportsDefaultKeyword","Does this backend support the bare `DEFAULT` keyword?"],["SupportsOnConflictClause","Does this backend support 'ON CONFLICT' clause?"],["SupportsOnConflictTargetDecorations","Does this backend support 'WHERE' clauses on 'ON CONFLICT' clauses?"],["SupportsReturningClause","A trait indicating that implementing backend provides support for `RETURNING` clauses."],["UsesAnsiSavepointSyntax","Does this backend use the standard `SAVEPOINT` syntax?"]],"type":[["RawValue","A helper type to get the raw representation of a database type given to `FromSql`. Equivalent to `<DB as Backend>::RawValue<'a>`."]]});
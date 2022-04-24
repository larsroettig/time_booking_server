table! {
    projects (id) {
        id -> Int4,
        abbreviation -> Varchar,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    time_booking (id) {
        id -> Int4,
        project_id -> Nullable<Int4>,
        description -> Varchar,
        start_at -> Timestamp,
        end_at -> Nullable<Timestamp>,
    }
}

joinable!(time_booking -> projects (project_id));

allow_tables_to_appear_in_same_query!(
    projects,
    time_booking,
);

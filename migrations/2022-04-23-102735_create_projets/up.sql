CREATE TABLE projects
(
    id           INT GENERATED ALWAYS AS IDENTITY,
    abbreviation VARCHAR(15) NOT NULL,
    name         VARCHAR(30) NOT NULL,
    description VARCHAR(255),
    PRIMARY KEY (id),
    UNIQUE (abbreviation)
);

CREATE TABLE time_booking
(
    id          INT GENERATED ALWAYS AS IDENTITY,
    project_id  INT,
    description VARCHAR(255) NOT NULL,
    start_at    TIMESTAMP    NOT NULL,
    end_at      TIMESTAMP,
    PRIMARY KEY (id),
    CONSTRAINT fk_project
        FOREIGN KEY (project_id)
            REFERENCES projects (id)
);
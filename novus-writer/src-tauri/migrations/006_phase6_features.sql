-- Migration for Phase 6: Advanced Collaboration & Productivity Features

-- Collaboration Sessions Table
CREATE TABLE IF NOT EXISTS collaboration_sessions (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

-- Connected Users Table
CREATE TABLE IF NOT EXISTS connected_users (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    username TEXT NOT NULL,
    cursor_position TEXT,
    color TEXT NOT NULL,
    joined_at DATETIME NOT NULL,
    last_activity DATETIME NOT NULL,
    FOREIGN KEY (session_id) REFERENCES collaboration_sessions(id) ON DELETE CASCADE,
    UNIQUE(session_id, user_id)
);

-- Templates Tables
CREATE TABLE IF NOT EXISTS template_categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    icon TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS templates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL,
    content TEXT NOT NULL,
    thumbnail TEXT,
    is_system BOOLEAN NOT NULL DEFAULT FALSE,
    created_by TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    usage_count INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (category) REFERENCES template_categories(name) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS template_variables (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    name TEXT NOT NULL,
    field_type TEXT NOT NULL,
    required BOOLEAN NOT NULL DEFAULT TRUE,
    default_value TEXT,
    options TEXT,
    description TEXT,
    FOREIGN KEY (template_id) REFERENCES templates(id) ON DELETE CASCADE,
    UNIQUE(template_id, name)
);

-- Citations Tables
CREATE TABLE IF NOT EXISTS citations (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    citation_type TEXT NOT NULL,
    entry_type TEXT NOT NULL,
    fields TEXT NOT NULL,
    formatted_citation TEXT NOT NULL,
    bibliography_entry TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_citations_document ON citations(document_id);

-- Mail Merge Tables
CREATE TABLE IF NOT EXISTS mail_merge_recipients_list (
    id TEXT PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT,
    address_line1 TEXT,
    address_line2 TEXT,
    city TEXT,
    state TEXT,
    postal_code TEXT,
    country TEXT,
    custom_fields TEXT,
    created_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS mail_merge_jobs (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    template_content TEXT NOT NULL,
    status TEXT NOT NULL,
    total_recipients INTEGER NOT NULL,
    processed_count INTEGER NOT NULL DEFAULT 0,
    created_by TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    completed_at DATETIME,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS mail_merge_recipients (
    job_id TEXT NOT NULL,
    recipient_id TEXT NOT NULL,
    PRIMARY KEY (job_id, recipient_id),
    FOREIGN KEY (job_id) REFERENCES mail_merge_jobs(id) ON DELETE CASCADE,
    FOREIGN KEY (recipient_id) REFERENCES mail_merge_recipients_list(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS mail_merge_results (
    id TEXT PRIMARY KEY,
    job_id TEXT NOT NULL,
    recipient_id TEXT NOT NULL,
    output_document_id TEXT,
    status TEXT NOT NULL,
    error_message TEXT,
    created_at DATETIME NOT NULL,
    FOREIGN KEY (job_id) REFERENCES mail_merge_jobs(id) ON DELETE CASCADE
);

-- Insert default template categories
INSERT OR IGNORE INTO template_categories (id, name, description, icon, sort_order) VALUES
    ('cat_1', 'Business Letters', 'Professional business correspondence', 'briefcase', 1),
    ('cat_2', 'Legal Documents', 'Legal forms and contracts', 'scale', 2),
    ('cat_3', 'Academic', 'Research papers and academic formats', 'graduation-cap', 3),
    ('cat_4', 'Personal', 'Personal letters and invitations', 'user', 4),
    ('cat_5', 'Reports', 'Business and technical reports', 'file-text', 5),
    ('cat_6', 'Resumes', 'CV and resume templates', 'user-circle', 6);

-- Insert sample templates
INSERT OR IGNORE INTO templates (id, name, description, category, content, is_system, created_at, updated_at, usage_count) VALUES
    ('tpl_1', 'Business Letter', 'Standard business letter format', 'Business Letters', 
     '{{date}}

{{recipient_name}}
{{recipient_title}}
{{company_name}}
{{address_line1}}
{{address_line2}}

Dear {{recipient_name}},

{{body}}

Sincerely,

{{sender_name}}
{{sender_title}}',
     TRUE, datetime('now'), datetime('now'), 0),
    
    ('tpl_2', 'Memo', 'Internal memorandum format', 'Business Letters',
     'MEMORANDUM

TO: {{to}}
FROM: {{from}}
DATE: {{date}}
SUBJECT: {{subject}}

{{body}}',
     TRUE, datetime('now'), datetime('now'), 0),
    
    ('tpl_3', 'APA Research Paper', 'APA 7th edition research paper template', 'Academic',
     '{{title}}

{{author_name}}
{{institution}}

{{abstract}}

{{body}}

References

{{bibliography}}',
     TRUE, datetime('now'), datetime('now'), 0);

-- Create index for better query performance
CREATE INDEX IF NOT EXISTS idx_templates_category ON templates(category);
CREATE INDEX IF NOT EXISTS idx_mail_merge_jobs_document ON mail_merge_jobs(document_id);
CREATE INDEX IF NOT EXISTS idx_mail_merge_jobs_status ON mail_merge_jobs(status);

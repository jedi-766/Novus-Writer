import React, { useState, useEffect } from 'react';
import { Users, Mail, FileText, Play, Pause, XCircle, CheckCircle } from 'lucide-react';

interface Recipient {
  id: string;
  firstName: string;
  lastName: string;
  email?: string;
  customFields: Record<string, string>;
}

interface MailMergeJob {
  id: string;
  documentId: string;
  status: 'draft' | 'processing' | 'completed' | 'failed' | 'cancelled';
  totalRecipients: number;
  processedCount: number;
  createdAt: string;
}

interface MailMergeManagerProps {
  documentId: string;
  documentContent: string;
}

const MailMergeManager: React.FC<MailMergeManagerProps> = ({ documentId, documentContent }) => {
  const [recipients, setRecipients] = useState<Recipient[]>([]);
  const [jobs, setJobs] = useState<MailMergeJob[]>([]);
  const [showAddRecipient, setShowAddRecipient] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [currentJob, setCurrentJob] = useState<MailMergeJob | null>(null);
  
  const [newRecipient, setNewRecipient] = useState({
    firstName: '',
    lastName: '',
    email: '',
    company: '',
    title: '',
  });

  useEffect(() => {
    fetchJobs();
  }, [documentId]);

  const fetchJobs = async () => {
    try {
      const response = await fetch(`/api/mailmerge/jobs?document_id=${documentId}`);
      if (response.ok) {
        const data = await response.json();
        setJobs(data);
      }
    } catch (error) {
      console.error('Failed to fetch jobs:', error);
    }
  };

  const handleAddRecipient = async () => {
    try {
      const response = await fetch('/api/mailmerge/recipients', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          first_name: newRecipient.firstName,
          last_name: newRecipient.lastName,
          email: newRecipient.email || undefined,
          custom_fields: {
            company: newRecipient.company,
            title: newRecipient.title,
          },
        }),
      });

      if (response.ok) {
        const recipient = await response.json();
        setRecipients([...recipients, recipient]);
        setShowAddRecipient(false);
        setNewRecipient({ firstName: '', lastName: '', email: '', company: '', title: '' });
      }
    } catch (error) {
      console.error('Failed to add recipient:', error);
    }
  };

  const handleStartMailMerge = async () => {
    if (recipients.length === 0) {
      alert('Please add at least one recipient');
      return;
    }

    setIsProcessing(true);
    
    try {
      const response = await fetch('/api/mailmerge/jobs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          document_id: documentId,
          template_content: documentContent,
          recipient_ids: recipients.map(r => r.id),
          created_by: 'current_user', // Replace with actual user ID
        }),
      });

      if (response.ok) {
        const job = await response.json();
        setCurrentJob(job);
        
        // Start processing
        await processJob(job.id);
      }
    } catch (error) {
      console.error('Failed to start mail merge:', error);
      setIsProcessing(false);
    }
  };

  const processJob = async (jobId: string) => {
    try {
      const response = await fetch(`/api/mailmerge/jobs/${jobId}/process`, {
        method: 'POST',
      });

      if (response.ok) {
        const results = await response.json();
        setCurrentJob({ ...currentJob!, status: 'completed', processedCount: results.length });
        await fetchJobs();
      }
    } catch (error) {
      console.error('Failed to process job:', error);
      if (currentJob) {
        setCurrentJob({ ...currentJob, status: 'failed' });
      }
    } finally {
      setIsProcessing(false);
    }
  };

  const handleCancelJob = async () => {
    if (currentJob) {
      try {
        await fetch(`/api/mailmerge/jobs/${currentJob.id}/cancel`, {
          method: 'POST',
        });
        setCurrentJob({ ...currentJob, status: 'cancelled' });
      } catch (error) {
        console.error('Failed to cancel job:', error);
      } finally {
        setIsProcessing(false);
      }
    }
  };

  const importCSV = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    // Simple CSV parsing - in production, use a proper CSV library
    const text = await file.text();
    const lines = text.split('\n');
    const headers = lines[0].split(',').map(h => h.trim().toLowerCase());
    
    const newRecipients: Recipient[] = [];
    for (let i = 1; i < lines.length; i++) {
      const values = lines[i].split(',').map(v => v.trim());
      if (values.length > 0 && values[0]) {
        const recipient: any = {
          firstName: values[headers.indexOf('firstname')] || values[headers.indexOf('first_name')] || '',
          lastName: values[headers.indexOf('lastname')] || values[headers.indexOf('last_name')] || '',
          email: values[headers.indexOf('email')] || '',
          customFields: {},
        };
        
        if (headers.includes('company')) {
          recipient.customFields.company = values[headers.indexOf('company')] || '';
        }
        
        newRecipients.push(recipient);
      }
    }
    
    setRecipients([...recipients, ...newRecipients]);
    event.target.value = '';
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'completed':
        return <CheckCircle className="w-4 h-4 text-green-600" />;
      case 'processing':
        return <Play className="w-4 h-4 text-blue-600 animate-pulse" />;
      case 'failed':
        return <XCircle className="w-4 h-4 text-red-600" />;
      case 'cancelled':
        return <XCircle className="w-4 h-4 text-gray-600" />;
      default:
        return <Pause className="w-4 h-4 text-gray-400" />;
    }
  };

  return (
    <div className="h-full flex flex-col">
      {/* Header */}
      <div className="p-4 border-b">
        <h2 className="text-lg font-semibold mb-3 flex items-center gap-2">
          <Mail className="w-5 h-5" />
          Mail Merge
        </h2>

        <p className="text-xs text-gray-600 mb-3">
          Create personalized documents by merging this template with recipient data.
          Use placeholders like {'{{first_name}}'}, {'{{last_name}}'}, {'{{email}}'}, etc.
        </p>

        {/* Add Recipient Button */}
        <button
          onClick={() => setShowAddRecipient(!showAddRecipient)}
          className="w-full px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 text-sm flex items-center justify-center gap-2 mb-2"
        >
          <Users className="w-4 h-4" />
          Add Recipient
        </button>

        {/* Import CSV */}
        <label className="w-full px-3 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 text-sm flex items-center justify-center gap-2 cursor-pointer">
          <FileText className="w-4 h-4" />
          Import CSV
          <input
            type="file"
            accept=".csv"
            onChange={importCSV}
            className="hidden"
          />
        </label>
      </div>

      {/* Add Recipient Form */}
      {showAddRecipient && (
        <div className="p-4 border-b bg-gray-50">
          <h3 className="text-sm font-medium mb-3">New Recipient</h3>
          
          <div className="space-y-2">
            <div className="grid grid-cols-2 gap-2">
              <input
                type="text"
                placeholder="First Name"
                value={newRecipient.firstName}
                onChange={(e) => setNewRecipient({ ...newRecipient, firstName: e.target.value })}
                className="px-2 py-1.5 border rounded text-sm"
              />
              <input
                type="text"
                placeholder="Last Name"
                value={newRecipient.lastName}
                onChange={(e) => setNewRecipient({ ...newRecipient, lastName: e.target.value })}
                className="px-2 py-1.5 border rounded text-sm"
              />
            </div>
            <input
              type="email"
              placeholder="Email"
              value={newRecipient.email}
              onChange={(e) => setNewRecipient({ ...newRecipient, email: e.target.value })}
              className="w-full px-2 py-1.5 border rounded text-sm"
            />
            <input
              type="text"
              placeholder="Company"
              value={newRecipient.company}
              onChange={(e) => setNewRecipient({ ...newRecipient, company: e.target.value })}
              className="w-full px-2 py-1.5 border rounded text-sm"
            />
            <input
              type="text"
              placeholder="Title"
              value={newRecipient.title}
              onChange={(e) => setNewRecipient({ ...newRecipient, title: e.target.value })}
              className="w-full px-2 py-1.5 border rounded text-sm"
            />
            
            <div className="flex gap-2 pt-2">
              <button
                onClick={handleAddRecipient}
                className="flex-1 px-3 py-1.5 bg-blue-600 text-white text-sm rounded hover:bg-blue-700"
              >
                Save
              </button>
              <button
                onClick={() => setShowAddRecipient(false)}
                className="flex-1 px-3 py-1.5 bg-gray-200 text-gray-700 text-sm rounded hover:bg-gray-300"
              >
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Recipients Count & Start Button */}
      <div className="p-4 border-b">
        <div className="flex items-center justify-between mb-3">
          <span className="text-sm text-gray-600">
            {recipients.length} recipient{recipients.length !== 1 ? 's' : ''}
          </span>
          {recipients.length > 0 && (
            <button
              onClick={handleStartMailMerge}
              disabled={isProcessing}
              className="px-4 py-2 bg-green-600 text-white text-sm rounded hover:bg-green-700 disabled:opacity-50 flex items-center gap-2"
            >
              <Play className="w-4 h-4" />
              Start Merge
            </button>
          )}
        </div>

        {/* Progress */}
        {isProcessing && currentJob && (
          <div className="bg-blue-50 rounded-md p-3">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm text-blue-800">Processing...</span>
              <button
                onClick={handleCancelJob}
                className="text-red-600 hover:text-red-800 text-xs"
              >
                Cancel
              </button>
            </div>
            <div className="w-full bg-blue-200 rounded-full h-2">
              <div
                className="bg-blue-600 h-2 rounded-full transition-all"
                style={{ width: `${(currentJob.processedCount / currentJob.totalRecipients) * 100}%` }}
              ></div>
            </div>
            <p className="text-xs text-blue-700 mt-1">
              {currentJob.processedCount} of {currentJob.totalRecipients} completed
            </p>
          </div>
        )}
      </div>

      {/* Recipients List */}
      <div className="flex-1 overflow-y-auto p-4">
        <h3 className="text-sm font-medium text-gray-700 mb-2">Recipients</h3>
        
        {recipients.length === 0 ? (
          <div className="text-center text-gray-500 py-8">
            <Users className="w-12 h-12 mx-auto mb-2 opacity-50" />
            <p>No recipients yet</p>
            <p className="text-xs mt-1">Add recipients manually or import from CSV</p>
          </div>
        ) : (
          <div className="space-y-2">
            {recipients.map((recipient) => (
              <div
                key={recipient.id}
                className="border rounded-lg p-3 bg-white"
              >
                <div className="flex items-center gap-3">
                  <div className="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center text-blue-700 font-medium text-sm">
                    {recipient.firstName[0]}{recipient.lastName[0]}
                  </div>
                  <div className="flex-1">
                    <p className="text-sm font-medium text-gray-900">
                      {recipient.firstName} {recipient.lastName}
                    </p>
                    {recipient.email && (
                      <p className="text-xs text-gray-500">{recipient.email}</p>
                    )}
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Job History */}
      {jobs.length > 0 && (
        <div className="border-t p-4 bg-gray-50">
          <h3 className="text-sm font-medium text-gray-700 mb-2">Recent Jobs</h3>
          <div className="space-y-2 max-h-32 overflow-y-auto">
            {jobs.slice(0, 5).map((job) => (
              <div
                key={job.id}
                className="flex items-center gap-2 text-xs p-2 bg-white rounded border"
              >
                {getStatusIcon(job.status)}
                <span className="flex-1 capitalize">{job.status}</span>
                <span className="text-gray-500">
                  {job.processedCount}/{job.totalRecipients}
                </span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default MailMergeManager;

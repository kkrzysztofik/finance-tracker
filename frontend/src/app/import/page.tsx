"use client";

import { useCallback, useState } from "react";
import { useDropzone } from "react-dropzone";
import { Upload, FileText, CheckCircle2, AlertCircle, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { importFile } from "@/lib/api";
import type { ImportResult } from "@/lib/types";

export default function ImportPage() {
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [uploading, setUploading] = useState(false);
  const [result, setResult] = useState<ImportResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const onDrop = useCallback((acceptedFiles: File[]) => {
    if (acceptedFiles.length > 0) {
      setSelectedFile(acceptedFiles[0]);
      setResult(null);
      setError(null);
    }
  }, []);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    accept: {
      "text/csv": [".csv"],
    },
    maxFiles: 1,
  });

  const handleUpload = async () => {
    if (!selectedFile) return;

    setUploading(true);
    setResult(null);
    setError(null);

    try {
      const importResult = await importFile(selectedFile);
      setResult(importResult);
      setSelectedFile(null);
    } catch (err) {
      const message =
        err instanceof Error ? err.message : "An unexpected error occurred.";
      setError(message);
    } finally {
      setUploading(false);
    }
  };

  const clearSelection = () => {
    setSelectedFile(null);
    setResult(null);
    setError(null);
  };

  return (
    <div className="mx-auto max-w-2xl space-y-6 p-6">
      <h1 className="text-2xl font-semibold tracking-tight">
        Import Transactions
      </h1>

      {/* Dropzone card */}
      <Card>
        <CardHeader>
          <CardTitle>Upload CSV File</CardTitle>
          <CardDescription>
            Drag and drop a bank statement CSV file, or click to browse.
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div
            {...getRootProps()}
            className={`flex min-h-[180px] cursor-pointer flex-col items-center justify-center gap-3 rounded-lg border-2 border-dashed p-8 text-center transition-colors ${
              isDragActive
                ? "border-primary bg-primary/5"
                : "border-muted-foreground/25 hover:border-primary/50 hover:bg-muted/50"
            }`}
            role="button"
            aria-label="Drop zone for CSV files"
          >
            <input {...getInputProps()} />
            <Upload
              className={`size-10 ${
                isDragActive
                  ? "text-primary"
                  : "text-muted-foreground"
              }`}
            />
            {isDragActive ? (
              <p className="text-sm font-medium text-primary">
                Drop the file here...
              </p>
            ) : (
              <>
                <p className="text-sm font-medium">
                  Drag and drop a CSV file here
                </p>
                <p className="text-xs text-muted-foreground">
                  or click to select a file
                </p>
              </>
            )}
          </div>

          {/* Selected file display */}
          {selectedFile && (
            <div className="flex items-center justify-between rounded-lg border bg-muted/50 px-4 py-3">
              <div className="flex items-center gap-3">
                <FileText className="size-5 text-muted-foreground" />
                <div>
                  <p className="text-sm font-medium">{selectedFile.name}</p>
                  <p className="text-xs text-muted-foreground">
                    {(selectedFile.size / 1024).toFixed(1)} KB
                  </p>
                </div>
              </div>
              <div className="flex items-center gap-2">
                <Button
                  variant="ghost"
                  size="icon-xs"
                  onClick={clearSelection}
                  aria-label="Remove selected file"
                >
                  <X className="size-4" />
                </Button>
              </div>
            </div>
          )}

          {/* Upload button */}
          <Button
            onClick={handleUpload}
            disabled={!selectedFile || uploading}
            className="w-full"
          >
            {uploading ? "Uploading..." : "Upload and Import"}
          </Button>

          {/* Success result */}
          {result && (
            <div className="flex items-start gap-3 rounded-lg border border-green-200 bg-green-50 p-4 dark:border-green-900 dark:bg-green-950/50">
              <CheckCircle2 className="mt-0.5 size-5 shrink-0 text-green-600 dark:text-green-400" />
              <div className="space-y-1">
                <p className="text-sm font-medium text-green-800 dark:text-green-200">
                  Import completed successfully
                </p>
                <div className="flex flex-wrap gap-2">
                  <Badge variant="secondary">
                    {result.total_rows} total rows
                  </Badge>
                  <Badge variant="secondary">
                    {result.imported} imported
                  </Badge>
                  <Badge variant="secondary">
                    {result.skipped} skipped
                  </Badge>
                </div>
              </div>
            </div>
          )}

          {/* Error message */}
          {error && (
            <div className="flex items-start gap-3 rounded-lg border border-red-200 bg-red-50 p-4 dark:border-red-900 dark:bg-red-950/50">
              <AlertCircle className="mt-0.5 size-5 shrink-0 text-red-600 dark:text-red-400" />
              <div className="space-y-1">
                <p className="text-sm font-medium text-red-800 dark:text-red-200">
                  Import failed
                </p>
                <p className="text-sm text-red-700 dark:text-red-300">
                  {error}
                </p>
              </div>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Supported formats card */}
      <Card>
        <CardHeader>
          <CardTitle>Supported Formats</CardTitle>
          <CardDescription>
            The following bank statement CSV formats are recognized
            automatically.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            <div className="flex items-start gap-3">
              <Badge variant="outline" className="mt-0.5 shrink-0">
                Alior
              </Badge>
              <div>
                <p className="text-sm font-medium">Alior Bank</p>
                <p className="text-xs text-muted-foreground">
                  File pattern:{" "}
                  <code className="rounded bg-muted px-1 py-0.5 font-mono">
                    Historia_Operacji_*.csv
                  </code>
                </p>
              </div>
            </div>

            <div className="flex items-start gap-3">
              <Badge variant="outline" className="mt-0.5 shrink-0">
                Pekao
              </Badge>
              <div>
                <p className="text-sm font-medium">Pekao SA</p>
                <p className="text-xs text-muted-foreground">
                  File pattern:{" "}
                  <code className="rounded bg-muted px-1 py-0.5 font-mono">
                    Lista_operacji_*.csv
                  </code>
                </p>
              </div>
            </div>

            <div className="flex items-start gap-3">
              <Badge variant="outline" className="mt-0.5 shrink-0">
                Revolut
              </Badge>
              <div>
                <p className="text-sm font-medium">Revolut</p>
                <p className="text-xs text-muted-foreground">
                  File pattern:{" "}
                  <code className="rounded bg-muted px-1 py-0.5 font-mono">
                    account-statement_*.csv
                  </code>
                </p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

{{/*
Expand the name of the chart.
*/}}
{{- define "epirust.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "epirust.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "epirust.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "epirust.labels" -}}
helm.sh/chart: {{ include "epirust.chart" . }}
{{ include "epirust.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "epirust.selectorLabels" -}}
app.kubernetes.io/name: {{ include "epirust.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "epirust.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "epirust.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}


{{- define "helpers.list-engine-env-variables"}}
{{- range $key, $val := .Values.engine.env }}
    - name: {{ $key }}
      value: {{ $val | quote }}
{{- end}}
{{- end }}


{{- define "helpers.list-orchestrator-env-variables"}}
{{- range $key, $val := .Values.orchestrator.env }}
    - name: {{ $key }}
      value: {{ $val }}
{{- end }}
{{- end }}

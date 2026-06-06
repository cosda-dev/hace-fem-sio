# FEM-SIO - Structured Intent Object

## Overview

FEM-SIO provides the Runtime Intermediate Representation layer between Brain and RAC communication.

## Structure

### Core Types

- **SioObject** - Core container with kind, schema, payload, attachments
- **SioKind** - Intent, Inference, Information, Command, Event, Response, Memory, Artifact
- **SioFormat** - Json, Yaml, Ail, Skb, Rcb, Gguf, Onnx, Binary
- **AttachmentUri** - Arena, SharedMemory, HostCall, Gguf, Onnx, Pl, Hc

### Schema Modules

- **header.rs** - SIO object headers
- **intent.rs** - Intent schema definitions
- **knowledge.rs** - Knowledge object schemas
- **ruleset.rs** - Policy ruleset schemas
- **reality.rs** - Reality schemas
- **runtime.rs** - Runtime schemas
- **context.rs** - Context schema
- **outcome.rs** - Outcome definitions

## Features

- UUID v4 for SioId
- Timestamp in metadata
- Attachment system for external resources

## Status

- **Phase**: Production ready
- **Contract**: BrainRuntime trait in manifest.ail
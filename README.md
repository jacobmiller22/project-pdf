
# Project PDF

Project PDF is a rust library for reading and writing PDFs following the PDF 2.0 Specification.

## Design Goals

The goal of Project PDF is to expose a series of interfaces that can abstract away the underlying
PDF specification in a manner that is performant.

This goal is in the context of allowing a memory safe, blazingly fast program to read and mutate PDF documents
and allow higher level libraries, written in general purpose programming languages, to provide a simple interface
to solve common PDF use cases. In many higher level programming languages, libraries exist to solve this problem, 
however many performant libraries that leverage FFI rely on closed source libraries and libraries that contain 
restrictive licenses incompatible for enterprise use.

## Target Use Cases

At the moment the following use cases are being prioritized:

1. Filling in form fields. Supporting various form field types including Text Fields, Signature Fields, 


## Why create a new PDF library when `pdfrs` exists?

As the author of this library, I began this project to learn more about the PDF specification and also have an 
excuse to further explore the rust programming language.

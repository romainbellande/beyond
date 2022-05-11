---
inject: true
to: libs/ui/src/lib/components/index.ts
after: exports
skip_if: export * from './<%=name%>';
eof_last: false
---
export * from './<%=name%>';

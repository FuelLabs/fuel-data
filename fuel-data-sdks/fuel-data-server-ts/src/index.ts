// src/index.ts
export async function lazyLoadModule() {
  const { hello } = await import('./module');
  console.log(hello('world'));
}

// Entry point
lazyLoadModule();

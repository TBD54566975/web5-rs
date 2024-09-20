export class Web5Error extends Error {
  variant: string;

  constructor(variant: string, message: string) {
    super(message);
    this.variant = variant;
    this.name = 'Web5Error'; 
  }
}

export const catchWeb5Error = (error: any): any => {
  if (error && typeof error === 'object' && 'variant' in error && 'message' in error) {
    return new Web5Error(error.variant, error.message);
  } 
  return error
}
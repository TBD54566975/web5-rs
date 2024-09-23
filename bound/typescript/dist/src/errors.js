export class Web5Error extends Error {
    constructor(variant, message) {
        super(message);
        this.variant = variant;
        this.name = 'Web5Error';
    }
}
export const catchWeb5Error = (error) => {
    if (error && typeof error === 'object' && error.is_web5_error) {
        return new Web5Error(error.variant, error.message);
    }
    return error;
};

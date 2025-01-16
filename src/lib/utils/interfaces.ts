type Decimal = number;

export type Entity = Category | Service | SubService;

export interface Category {
    id: number;
    name: string;
    services: Service[];
}

export interface Service {
    id: number;
    name: string;
    price?: Decimal;
    sub_services: SubService[];
}

export interface SubService {
    id: number;
    name: string;
    price?: Decimal;
}
export type Decimal = number;

// -------- Image Interfaces --------
export interface Image {
    name: string;
    width: number;
    height: number;
    size: number;
    url: string;
}

export interface ImageVariants {
    thumbnail?: Image;
    large?: Image;
    medium?: Image;
    small?: Image;
}

export interface ImageDetails {
    name: string;
    alternative_text: string;
    width: number;
    height: number;
    formats?: ImageVariants;
    size: number;
    url: string;
}

// -------- Social Media Interface --------
export interface SocialMedia {
    name: string;
    url: string;
    icon: ImageDetails;
}

// -------- Contact Interfaces --------
export interface Address {
    city: string;
    zip_code: string;
    street: string;
    house_number: string;
}

export interface ContactInformation {
    phone_number: string;
}

export interface OpeningHours {
    begin: string | null;
    end: string | null;
    closed: boolean;
    days: string;
}

export interface Contact {
    address: Address;
    contact_information: ContactInformation;
    opening_hours: OpeningHours[];
    social_media: SocialMedia[];
}

// -------- Service Interfaces --------
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

// -------- Constants --------
export const ENTRY_TYPES = {
    CATEGORY: "category",
    SERVICE: "service",
    SUB_SERVICE: "sub_service",
} as const;
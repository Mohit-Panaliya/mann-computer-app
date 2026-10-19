import { invoke } from '@tauri-apps/api/core';

export interface Customer {
  id: string;
  name: string;
  phone: string;
  address?: string;
  created_at: string;
}

export interface InwardSlip {
  id: string;
  slip_no: string;
  customer_id: string;
  customer_name?: string;
  customer_phone?: string;
  date: string;
  device_type: string;
  problem_description?: string;
  parts_checklist?: string;
  remarks?: string;
  status: string;
  created_at: string;
}

export interface QuotationItem {
  description: string;
  quantity: number;
  unit_price: number;
  total: number;
}

export interface Quotation {
  id: string;
  quotation_no: string;
  inward_id?: string;
  customer_id: string;
  customer_name?: string;
  items: QuotationItem[];
  total_amount: number;
  status: string;
  notes?: string;
  created_at: string;
}

export interface Repair {
  id: string;
  inward_id: string;
  quotation_id?: string;
  status: string;
  diagnosis?: string;
  parts_used?: string;
  labor_cost: number;
  total_cost: number;
  technician?: string;
  start_date?: string;
  end_date?: string;
  created_at: string;
}

export interface BillItem {
  description: string;
  quantity: number;
  unit_price: number;
  total: number;
}

export interface Bill {
  id: string;
  bill_no: string;
  inward_id?: string;
  repair_id?: string;
  customer_id: string;
  customer_name?: string;
  customer_phone?: string;
  items: BillItem[];
  subtotal: number;
  tax: number;
  total_amount: number;
  payment_status: string;
  payment_method?: string;
  created_at: string;
}

export interface Part {
  id: string;
  name: string;
  category?: string;
  description?: string;
  purchase_price: number;
  selling_price: number;
  stock_quantity: number;
  compatible_with?: string;
  created_at: string;
}

export interface CctvQuote {
  id: string;
  quote_no: string;
  customer_id?: string;
  customer_name?: string;
  customer_phone?: string;
  items: QuotationItem[];
  total_amount: number;
  status: string;
  notes?: string;
  created_at: string;
}

export interface LaptopQuote {
  id: string;
  quote_no: string;
  customer_id?: string;
  customer_name?: string;
  customer_phone?: string;
  specs: string;
  price: number;
  status: string;
  notes?: string;
  created_at: string;
}

export interface DeliveryRecord {
  id: string;
  inward_id: string;
  bill_id?: string;
  customer_id: string;
  delivered_at: string;
  delivered_by?: string;
  received_by?: string;
  notes?: string;
}

export interface DashboardStats {
  total_inward: number;
  pending_repair: number;
  ready_for_delivery: number;
  delivered: number;
  total_bills: number;
  pending_bills: number;
  today_revenue: number;
  parts_in_stock: number;
}

const async_invoke = invoke;

export const api = {
  getCustomers: () => async_invoke<Customer[]>('get_customers'),
  createCustomer: (name: string, phone: string, address?: string) =>
    async_invoke<Customer>('create_customer', { name, phone, address }),
  updateCustomer: (id: string, name: string, phone: string, address?: string) =>
    async_invoke<void>('update_customer', { id, name, phone, address }),

  getInwardSlips: () => async_invoke<InwardSlip[]>('get_inward_slips'),
  getInwardSlip: (id: string) => async_invoke<InwardSlip | null>('get_inward_slip', { id }),
  createInwardSlip: (customerId: string, deviceType: string, problem?: string, partsChecklist?: string, remarks?: string) =>
    async_invoke<InwardSlip>('create_inward_slip', { customerId, deviceType, problemDescription: problem, partsChecklist, remarks }),
  updateInwardSlipStatus: (id: string, status: string) =>
    async_invoke<void>('update_inward_slip_status', { id, status }),

  getQuotations: () => async_invoke<Quotation[]>('get_quotations'),
  createQuotation: (inwardId: string | null, customerId: string, items: QuotationItem[], notes?: string) =>
    async_invoke<Quotation>('create_quotation', { inwardId, customerId, items, notes }),

  getRepairs: () => async_invoke<Repair[]>('get_repairs'),
  createRepair: (inwardId: string, quotationId: string | null, diagnosis: string | null, laborCost: number, technician: string | null) =>
    async_invoke<Repair>('create_repair', { inwardId, quotationId, diagnosis, laborCost, technician }),
  updateRepair: (id: string, status: string, diagnosis: string | null, partsUsed: string | null, laborCost: number, totalCost: number, endDate: string | null) =>
    async_invoke<void>('update_repair', { id, status, diagnosis, partsUsed, laborCost, totalCost, endDate }),

  getBills: () => async_invoke<Bill[]>('get_bills'),
  createBill: (inwardId: string | null, repairId: string | null, customerId: string, items: BillItem[], paymentMethod?: string) =>
    async_invoke<Bill>('create_bill', { inwardId, repairId, customerId, items, paymentMethod }),
  updateBillPayment: (id: string, paymentStatus: string, paymentMethod?: string) =>
    async_invoke<void>('update_bill_payment', { id, paymentStatus, paymentMethod }),

  getParts: () => async_invoke<Part[]>('get_parts'),
  createPart: (name: string, category: string | null, description: string | null, purchasePrice: number, sellingPrice: number, stockQuantity: number, compatibleWith: string | null) =>
    async_invoke<Part>('create_part', { name, category, description, purchasePrice, sellingPrice, stockQuantity, compatibleWith }),
  updatePartStock: (id: string, quantity: number) =>
    async_invoke<void>('update_part_stock', { id, quantity }),
  getPartSales: () => async_invoke<any[]>('get_part_sales'),
  createPartSale: (partId: string, customerId: string | null, customerName: string | null, quantity: number, unitPrice: number) =>
    async_invoke<any>('create_part_sale', { partId, customerId, customerName, quantity, unitPrice }),

  getCctvQuotes: () => async_invoke<CctvQuote[]>('get_cctv_quotes'),
  createCctvQuote: (customerName: string, customerPhone: string, items: QuotationItem[], notes?: string) =>
    async_invoke<CctvQuote>('create_cctv_quote', { customerName, customerPhone, items, notes }),

  getLaptopQuotes: () => async_invoke<LaptopQuote[]>('get_laptop_quotes'),
  createLaptopQuote: (customerName: string, customerPhone: string, specs: string, price: number, notes?: string) =>
    async_invoke<LaptopQuote>('create_laptop_quote', { customerName, customerPhone, specs, price, notes }),

  createDelivery: (inwardId: string, billId: string | null, customerId: string, deliveredBy?: string, receivedBy?: string, notes?: string) =>
    async_invoke<DeliveryRecord>('create_delivery', { inwardId, billId, customerId, deliveredBy, receivedBy, notes }),

  getDashboardStats: () => async_invoke<DashboardStats>('get_dashboard_stats'),
  generatePdf: (docType: string, id: string) => async_invoke<string>('generate_pdf', { docType, id }),
  getSettings: () => async_invoke<Record<string, string>>('get_settings'),
  updateSetting: (key: string, value: string) => async_invoke<void>('update_setting', { key, value }),
};

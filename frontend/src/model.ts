export interface BlockData  {
    block_number: number,
    gas_used: number,
    total_transaction: number,
    transactions: Transaction[],
    total? : number
    usd?: number


}

export interface Transaction{
    value: string;
}



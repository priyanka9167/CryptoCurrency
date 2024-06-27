export interface BlockData  {
    block_hash: string;
    block_height: number; // Assuming block_height corresponds to block_number
    total_transaction: number;
    time: string; // Assuming you handle DateTime conversion on the frontend
    transaction_in_usd: number;


}




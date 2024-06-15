import React, { useEffect, useState } from 'react'
import { LineChart, Line, CartesianGrid, XAxis, YAxis, Tooltip, Legend, ResponsiveContainer } from 'recharts'
import { BlockData } from './model';
import './Charts.css';

const Charts: React.FC = () => {

    const [data, setData] = useState<Array<BlockData>>([]);
    const [rate, setRate] = useState<number>(3000);

    const fetchRate = async () => {
        try {
            let response = await fetch("http://localhost:3000/get_rate");
            let rate = await response.json();
            setRate(rate.ethereum.usd || 3000); // Set default rate if rateData.ethereum.usd is undefined
        }
        catch (e) {
            setRate(3000)
            // console.log(e)
        }
    }

    const fetchApi = async () => {
        try {
            let response = await fetch("http://localhost:3000/get_bitcoin");
            let block_data = await response.json();
            const parseData = block_data.map((block: BlockData) => {
                // Initialize transactionValues array
                let transactionValues: number[] = [];

                // Check if block.transactions is defined and not empty
                if (block.transactions && block.transactions.length > 0) {
                    // Map over transactions to extract values
                    transactionValues = block.transactions.map((data) => parseFloat(data.value));
                }

                // Calculate total value of transactions
                const total = BigInt(transactionValues.reduce((partialSum: number, value: number) => partialSum + value, 0));
                const weiPerEther = BigInt('1000000000000000000');
                let usdValue: number | undefined;

                if (rate !== undefined) {
                    const ethValue = total / weiPerEther;
                    usdValue = Number(ethValue) * rate;
                }
                return {
                    block_number: Number(block.block_number),
                    total_transaction: typeof block.total_transaction === 'string' ? parseInt(block.total_transaction, 10) : block.total_transaction,
                    gas_used: typeof block.gas_used === 'string' ? parseFloat(block.gas_used) : block.gas_used,
                    transactions: transactionValues,
                    total: total,
                    usdValue: usdValue
                }

            });

            setData(parseData);
        }
        catch (e) {
            console.log(e)
        }
    }

    const formatYAxis = (tickItem: number) => {
        if (tickItem >= 1000000) {
            return (tickItem / 1000000).toFixed(1) + 'M';
        }
        if (tickItem >= 1000) {
            return (tickItem / 1000).toFixed(1) + 'K';
        }
        return tickItem.toString();
    };

    useEffect(() => {
        fetchRate();
        fetchApi();
       
        const interval = setInterval(() => {
            fetchRate();
            fetchApi();
        }, 20000);

        return () => clearInterval(interval);

    }, [])

    useEffect(() => {
        console.log(data); // Debugging: Log the data
        console.log(rate);
    }, [data]);

    return (
        <div className="chart-container">
            <h1>Ethereum Charts</h1>
            {data.length > 0 ? (
                <>
                    <div className="chart">
                        <h2>Total Transactions</h2>
                        <ResponsiveContainer width="100%" height={400}>
                            <LineChart
                                data={data}
                                margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                            >
                                <CartesianGrid strokeDasharray="3 3" />
                                <XAxis dataKey="block_number" />
                                <YAxis tickFormatter={formatYAxis} />
                                <Tooltip />
                                <Legend />
                                <Line type="monotone" dataKey="total_transaction" stroke="#8884d8" activeDot={{ r: 8 }} />
                            </LineChart>
                        </ResponsiveContainer>
                    </div>
                    <div className="chart">
                        <h2>USD Value</h2>
                        <ResponsiveContainer width="100%" height={400}>
                            <LineChart
                                data={data}
                                margin={{ top: 20, right: 30, left: 20, bottom: 5 }}
                            >
                                <CartesianGrid strokeDasharray="3 3" />
                                <XAxis dataKey="block_number" />
                                <YAxis tickFormatter={formatYAxis} />
                                <Tooltip />
                                <Legend />
                                <Line type="monotone" dataKey="usdValue" stroke="#82ca9d" activeDot={{ r: 8 }} />
                            </LineChart>
                        </ResponsiveContainer>
                    </div>
                </>
            ) : (
                <p>Loading...</p>
            )}
        </div>
    )
}

export default Charts;

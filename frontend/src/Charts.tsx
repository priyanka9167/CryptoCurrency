import React, { useEffect, useState } from 'react'
import { LineChart, Line, CartesianGrid, XAxis, YAxis, Tooltip, Legend, ResponsiveContainer } from 'recharts'
import { BlockData } from './model';
import './Charts.css';

const Charts: React.FC = () => {

    const [data, setData] = useState<Array<BlockData>>([]);



    const fetchApi = async () => {
        try {
            let response = await fetch("http://localhost:3000/get_bitcoin");
            let block_data = await response.json();
            const parseData = block_data.map((block: BlockData) => {


                return {
                    block_hash: block.block_hash,
                    block_height: block.block_height,
                    total_transaction: block.total_transaction,
                    time: block.time,
                    transaction_in_usd: block.transaction_in_usd
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

        fetchApi();

        const interval = setInterval(() => {

            fetchApi();
        }, 20000);

        return () => clearInterval(interval);

    }, [])

    useEffect(() => {
        console.log(data); // Debugging: Log the data

    }, [data]);

    return (
        <div className="chart-container">
            <h1>Bitcoin Charts</h1>
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
                                <XAxis dataKey="block_height" />
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
                                <XAxis dataKey="block_height" />
                                <YAxis tickFormatter={formatYAxis} />
                                <Tooltip />
                                <Legend />
                                <Line type="monotone" dataKey="transaction_in_usd" stroke="#82ca9d" activeDot={{ r: 8 }} />
                            </LineChart>
                        </ResponsiveContainer>
                    </div>
                </>
            ) : (
                <p>Loading...</p>
            )}

            <div className="block-table-container">
                <h2>Block Data</h2>
                <table className="block-table">
                    <thead>
                        <tr>
                            <th>Block Hash</th>
                            <th>Block Height</th>
                            
                        </tr>
                    </thead>
                    <tbody>
                        {data.map((block, index) => (
                            <tr key={index}>
                                <td>{block.block_hash}</td>
                                <td>{block.block_height}</td>
                                
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    )
}

export default Charts;

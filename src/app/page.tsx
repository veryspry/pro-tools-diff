'use client'

import  { useEffect, useState } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import Image from 'next/image'

interface Drive {
  bus_number: number,
  address: number,
  vendor_id: number,
  product_id: number,
  product_name: String
}

export default function Home() {
  const [greeting, setGreeting] = useState<string>('')
  const [drives, setDrives] = useState<Drive[]>([])
  useEffect(() => {
    invoke<string>('greet', { name: 'Next.js' })
      .then((res) => {
        setGreeting(res)
      })
      .catch(console.error)

      invoke<Drive[]>('get_connected_drives')
        .then((res) => {
          console.log(res)
          setDrives(res)
        })
        .catch(console.error)
  }, [])

  return (
    <div className="flex col">
      <h1>Connected Drives:</h1>
      <br/>
      <br/>
      <ul>
        {drives.map((drive) => (
          <li key={drive.product_id}> 
            <p>{drive.product_name}</p>
          </li>
        ))}
      </ul>
    </div>
  )
}

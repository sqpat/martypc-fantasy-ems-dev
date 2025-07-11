/*
    MartyPC
    https://github.com/dbalsom/martypc

    Copyright 2022-2025 Daniel Balsom

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the “Software”),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.

    --------------------------------------------------------------------------
*/

//! Enum-based dispatch for devices on the system bus.

use crate::{
    bus::{BusInterface, DeviceRunTimeUnit, IoDevice, MemoryMappedDevice, NO_IO_BYTE, OPEN_BUS_BYTE},
    cpu_common::LogicAnalyzer,
    device_traits::videocard::VideoCardDispatch,
    devices::{
        conventional_memory::ConventionalMemory,
        hdc::{jr_ide::JrIdeController, xebec::HardDiskController, xtide::XtIdeController},
        lotech_ems::LotechEmsCard,
        fantasy_ems::FantasyEmsCard,
    },
};

pub enum HdcDispatch {
    Xebec(HardDiskController),
    XtIde(XtIdeController),
    JrIde(JrIdeController),
}

impl HdcDispatch {
    pub fn io_read_u8(&mut self, port: u16, delta: DeviceRunTimeUnit) -> u8 {
        match self {
            HdcDispatch::Xebec(hdc) => IoDevice::read_u8(hdc, port, delta),
            HdcDispatch::XtIde(xtide) => IoDevice::read_u8(xtide, port, delta),
            HdcDispatch::JrIde(jride) => IoDevice::read_u8(jride, port, delta),
        }
    }

    pub fn io_write_u8(
        &mut self,
        port: u16,
        data: u8,
        bus: Option<&mut BusInterface>,
        delta: DeviceRunTimeUnit,
        analyzer: Option<&mut LogicAnalyzer>,
    ) {
        match self {
            HdcDispatch::Xebec(hdc) => IoDevice::write_u8(hdc, port, data, bus, delta, analyzer),
            HdcDispatch::XtIde(xtide) => IoDevice::write_u8(xtide, port, data, bus, delta, analyzer),
            HdcDispatch::JrIde(jride) => IoDevice::write_u8(jride, port, data, bus, delta, analyzer),
        }
    }

    pub fn mmio_peek_u8(&self, address: usize, cpumem: Option<&[u8]>) -> u8 {
        match self {
            HdcDispatch::JrIde(jride) => MemoryMappedDevice::mmio_peek_u8(jride, address, cpumem),
            _ => OPEN_BUS_BYTE,
        }
    }

    pub fn mmio_read_u8(&mut self, address: usize, ticks: u32, cpumem: Option<&[u8]>) -> (u8, u32) {
        match self {
            HdcDispatch::JrIde(jride) => MemoryMappedDevice::mmio_read_u8(jride, address, ticks, cpumem),
            _ => (OPEN_BUS_BYTE, 0),
        }
    }

    pub fn mmio_write_u8(&mut self, address: usize, data: u8, ticks: u32, cpumem: Option<&mut [u8]>) -> u32 {
        match self {
            HdcDispatch::JrIde(jride) => MemoryMappedDevice::mmio_write_u8(jride, address, data, ticks, cpumem),
            _ => 0,
        }
    }
}

pub enum EmsDispatch {
    LoTech(Box<LotechEmsCard>),
    FantasyEms(Box<FantasyEmsCard>),
}

pub enum MemoryDispatch {
    Conventional(ConventionalMemory),
    Ems(LotechEmsCard),
    FantasyEms(FantasyEmsCard),
}

impl MemoryDispatch {
    pub fn mmio_peek_u8(&self, address: usize, cpumem: Option<&[u8]>) -> u8 {
        match self {
            MemoryDispatch::Conventional(conventional) => conventional.mmio_peek_u8(address, cpumem),
            MemoryDispatch::Ems(ems) => ems.mmio_peek_u8(address, cpumem),
            MemoryDispatch::FantasyEms(fantasy_ems) => fantasy_ems.mmio_peek_u8(address, cpumem),
        }
    }

    pub fn mmio_read_u8(&mut self, address: usize, ticks: u32, cpumem: Option<&[u8]>) -> (u8, u32) {
        match self {
            MemoryDispatch::Conventional(conventional) => conventional.mmio_read_u8(address, ticks, cpumem),
            MemoryDispatch::Ems(ems) => ems.mmio_read_u8(address, ticks, cpumem),
            MemoryDispatch::FantasyEms(fantasy_ems) => fantasy_ems.mmio_read_u8(address, ticks, cpumem),
        }
    }

    pub fn mmio_write_u8(&mut self, address: usize, data: u8, ticks: u32, cpumem: Option<&mut [u8]>) -> u32 {
        match self {
            MemoryDispatch::Conventional(conventional) => conventional.mmio_write_u8(address, data, ticks, cpumem),
            MemoryDispatch::Ems(ems) => ems.mmio_write_u8(address, data, ticks, cpumem),
            MemoryDispatch::FantasyEms(fantasy_ems) => fantasy_ems.mmio_write_u8(address, data, ticks, cpumem),
        }
    }

    pub fn mmio_read_wait(&mut self, address: usize, system_ticks: u32) -> u32 {
        match self {
            MemoryDispatch::Conventional(conventional) => conventional.get_read_wait(address, system_ticks),
            MemoryDispatch::Ems(ems) => ems.get_read_wait(address, system_ticks),
            MemoryDispatch::FantasyEms(fantasy_ems) => fantasy_ems.get_read_wait(address, system_ticks),
        }
    }
}

impl VideoCardDispatch {
    pub fn io_read_u8(&mut self, port: u16, delta: DeviceRunTimeUnit) -> u8 {
        match self {
            VideoCardDispatch::None => NO_IO_BYTE,
            VideoCardDispatch::Mda(mda) => IoDevice::read_u8(&mut **mda, port, delta),
            VideoCardDispatch::Cga(cga) => IoDevice::read_u8(&mut **cga, port, delta),
            VideoCardDispatch::Tga(tga) => IoDevice::read_u8(&mut **tga, port, delta),
            #[cfg(feature = "ega")]
            VideoCardDispatch::Ega(ega) => IoDevice::read_u8(&mut **ega, port, delta),
            #[cfg(feature = "vga")]
            VideoCardDispatch::Vga(vga) => IoDevice::read_u8(&mut **vga, port, delta),
        }
    }

    pub fn io_write_u8(
        &mut self,
        port: u16,
        data: u8,
        bus: Option<&mut BusInterface>,
        delta: DeviceRunTimeUnit,
        analyzer: Option<&mut LogicAnalyzer>,
    ) {
        match self {
            VideoCardDispatch::None => {}
            VideoCardDispatch::Mda(mda) => IoDevice::write_u8(&mut **mda, port, data, bus, delta, analyzer),
            VideoCardDispatch::Cga(cga) => IoDevice::write_u8(&mut **cga, port, data, bus, delta, analyzer),
            VideoCardDispatch::Tga(tga) => IoDevice::write_u8(&mut **tga, port, data, bus, delta, analyzer),
            #[cfg(feature = "ega")]
            VideoCardDispatch::Ega(ega) => IoDevice::write_u8(&mut **ega, port, data, bus, delta, analyzer),
            #[cfg(feature = "vga")]
            VideoCardDispatch::Vga(vga) => IoDevice::write_u8(&mut **vga, port, data, bus, delta, analyzer),
        }
    }

    pub fn mmio_peek_u8(&self, address: usize, cpumem: Option<&[u8]>) -> u8 {
        match self {
            VideoCardDispatch::None => NO_IO_BYTE,
            VideoCardDispatch::Mda(mda) => MemoryMappedDevice::mmio_peek_u8(&**mda, address, cpumem),
            VideoCardDispatch::Cga(cga) => MemoryMappedDevice::mmio_peek_u8(&**cga, address, cpumem),
            VideoCardDispatch::Tga(tga) => MemoryMappedDevice::mmio_peek_u8(&**tga, address, cpumem),
            #[cfg(feature = "ega")]
            VideoCardDispatch::Ega(ega) => MemoryMappedDevice::mmio_peek_u8(&**ega, address, cpumem),
            #[cfg(feature = "vga")]
            VideoCardDispatch::Vga(vga) => MemoryMappedDevice::mmio_peek_u8(&**vga, address, cpumem),
        }
    }

    pub fn mmio_read_wait(&mut self, address: usize, system_ticks: u32) -> u32 {
        match self {
            VideoCardDispatch::None => 0,
            VideoCardDispatch::Mda(mda) => mda.get_read_wait(address, system_ticks),
            VideoCardDispatch::Cga(cga) => cga.get_read_wait(address, system_ticks),
            VideoCardDispatch::Tga(tga) => tga.get_read_wait(address, system_ticks),
            #[cfg(feature = "ega")]
            VideoCardDispatch::Ega(ega) => ega.get_read_wait(address, system_ticks),
            #[cfg(feature = "vga")]
            VideoCardDispatch::Vga(vga) => vga.get_read_wait(address, system_ticks),
        }
    }

    pub fn mmio_write_wait(&mut self, address: usize, system_ticks: u32) -> u32 {
        match self {
            VideoCardDispatch::None => 0,
            VideoCardDispatch::Mda(mda) => mda.get_write_wait(address, system_ticks),
            VideoCardDispatch::Cga(cga) => cga.get_write_wait(address, system_ticks),
            VideoCardDispatch::Tga(tga) => tga.get_write_wait(address, system_ticks),
            #[cfg(feature = "ega")]
            VideoCardDispatch::Ega(ega) => ega.get_write_wait(address, system_ticks),
            #[cfg(feature = "vga")]
            VideoCardDispatch::Vga(vga) => vga.get_write_wait(address, system_ticks),
        }
    }

    pub fn mmio_read_u8(&mut self, address: usize, ticks: u32, cpumem: Option<&[u8]>) -> (u8, u32) {
        match self {
            VideoCardDispatch::None => (NO_IO_BYTE, 0),
            VideoCardDispatch::Mda(mda) => MemoryMappedDevice::mmio_read_u8(&mut **mda, address, ticks, cpumem),
            VideoCardDispatch::Cga(cga) => MemoryMappedDevice::mmio_read_u8(&mut **cga, address, ticks, cpumem),
            VideoCardDispatch::Tga(tga) => MemoryMappedDevice::mmio_read_u8(&mut **tga, address, ticks, cpumem),
            #[cfg(feature = "ega")]
            VideoCardDispatch::Ega(ega) => MemoryMappedDevice::mmio_read_u8(&mut **ega, address, ticks, cpumem),
            #[cfg(feature = "vga")]
            VideoCardDispatch::Vga(vga) => MemoryMappedDevice::mmio_read_u8(&mut **vga, address, ticks, cpumem),
        }
    }

    pub fn mmio_read_u16(&mut self, address: usize, ticks: u32, cpumem: Option<&[u8]>) -> (u16, u32) {
        match self {
            VideoCardDispatch::None => (0, 0),
            VideoCardDispatch::Mda(mda) => MemoryMappedDevice::mmio_read_u16(&mut **mda, address, ticks, cpumem),
            VideoCardDispatch::Cga(cga) => MemoryMappedDevice::mmio_read_u16(&mut **cga, address, ticks, cpumem),
            VideoCardDispatch::Tga(tga) => MemoryMappedDevice::mmio_read_u16(&mut **tga, address, ticks, cpumem),
            #[cfg(feature = "ega")]
            VideoCardDispatch::Ega(ega) => MemoryMappedDevice::mmio_read_u16(&mut **ega, address, ticks, cpumem),
            #[cfg(feature = "vga")]
            VideoCardDispatch::Vga(vga) => MemoryMappedDevice::mmio_read_u16(&mut **vga, address, ticks, cpumem),
        }
    }

    pub fn mmio_write_u8(&mut self, address: usize, data: u8, ticks: u32, cpumem: Option<&mut [u8]>) -> u32 {
        match self {
            VideoCardDispatch::None => 0,
            VideoCardDispatch::Mda(mda) => MemoryMappedDevice::mmio_write_u8(&mut **mda, address, data, ticks, cpumem),
            VideoCardDispatch::Cga(cga) => MemoryMappedDevice::mmio_write_u8(&mut **cga, address, data, ticks, cpumem),
            VideoCardDispatch::Tga(tga) => MemoryMappedDevice::mmio_write_u8(&mut **tga, address, data, ticks, cpumem),
            #[cfg(feature = "ega")]
            VideoCardDispatch::Ega(ega) => MemoryMappedDevice::mmio_write_u8(&mut **ega, address, data, ticks, cpumem),
            #[cfg(feature = "vga")]
            VideoCardDispatch::Vga(vga) => MemoryMappedDevice::mmio_write_u8(&mut **vga, address, data, ticks, cpumem),
        }
    }

    pub fn mmio_write_u16(&mut self, address: usize, data: u16, ticks: u32, cpumem: Option<&mut [u8]>) -> u32 {
        match self {
            VideoCardDispatch::None => 0,
            VideoCardDispatch::Mda(mda) => MemoryMappedDevice::mmio_write_u16(&mut **mda, address, data, ticks, cpumem),
            VideoCardDispatch::Cga(cga) => MemoryMappedDevice::mmio_write_u16(&mut **cga, address, data, ticks, cpumem),
            VideoCardDispatch::Tga(tga) => MemoryMappedDevice::mmio_write_u16(&mut **tga, address, data, ticks, cpumem),
            #[cfg(feature = "ega")]
            VideoCardDispatch::Ega(ega) => MemoryMappedDevice::mmio_write_u16(&mut **ega, address, data, ticks, cpumem),
            #[cfg(feature = "vga")]
            VideoCardDispatch::Vga(vga) => MemoryMappedDevice::mmio_write_u16(&mut **vga, address, data, ticks, cpumem),
        }
    }
}

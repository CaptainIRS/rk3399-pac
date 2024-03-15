#[doc = "Register `PCIE_CLIENT_SIDE_BAND_STATUS` reader"]
pub type R = crate::R<PcieClientSideBandStatusSpec>;
#[doc = "PIPE phy de-emphasis status Transmitter de-emphasis selection, it combined by tx_deemphasis_ext,tx_deemphasis.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxDeemphasis {
    #[doc = "0: -3dB de-emphasis"]
    B000 = 0,
    #[doc = "1: -3dB de-emphasis"]
    B001 = 1,
    #[doc = "2: -3dB de-emphasis"]
    B010 = 2,
    #[doc = "3: -3dB de-emphasis"]
    B011 = 3,
    #[doc = "4: -3dB de-emphasis"]
    B100 = 4,
    #[doc = "5: -3dB de-emphasis"]
    B101 = 5,
    #[doc = "6: -3dB de-emphasis"]
    B110 = 6,
    #[doc = "7: -3dB de-emphasis"]
    B111 = 7,
}
impl From<TxDeemphasis> for u8 {
    #[inline(always)]
    fn from(variant: TxDeemphasis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TxDeemphasis {
    type Ux = u8;
}
#[doc = "Field `TX_DEEMPHASIS` reader - PIPE phy de-emphasis status Transmitter de-emphasis selection, it combined by tx_deemphasis_ext,tx_deemphasis."]
pub type TxDeemphasisR = crate::FieldReader<TxDeemphasis>;
impl TxDeemphasisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxDeemphasis {
        match self.bits {
            0 => TxDeemphasis::B000,
            1 => TxDeemphasis::B001,
            2 => TxDeemphasis::B010,
            3 => TxDeemphasis::B011,
            4 => TxDeemphasis::B100,
            5 => TxDeemphasis::B101,
            6 => TxDeemphasis::B110,
            7 => TxDeemphasis::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "-3dB de-emphasis"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == TxDeemphasis::B000
    }
    #[doc = "-3dB de-emphasis"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == TxDeemphasis::B001
    }
    #[doc = "-3dB de-emphasis"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == TxDeemphasis::B010
    }
    #[doc = "-3dB de-emphasis"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == TxDeemphasis::B011
    }
    #[doc = "-3dB de-emphasis"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == TxDeemphasis::B100
    }
    #[doc = "-3dB de-emphasis"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == TxDeemphasis::B101
    }
    #[doc = "-3dB de-emphasis"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == TxDeemphasis::B110
    }
    #[doc = "-3dB de-emphasis"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == TxDeemphasis::B111
    }
}
#[doc = "RX standby status Indicates PHY's RxStandby state\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RxStandbySt {
    #[doc = "0: Standby Always high during P1/P2/L1SS state."]
    B0 = 0,
    #[doc = "1: Standby Always high during P1/P2/L1SS state."]
    B1 = 1,
}
impl From<RxStandbySt> for u8 {
    #[inline(always)]
    fn from(variant: RxStandbySt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RxStandbySt {
    type Ux = u8;
}
#[doc = "Field `RX_STANDBY_ST` reader - RX standby status Indicates PHY's RxStandby state"]
pub type RxStandbyStR = crate::FieldReader<RxStandbySt>;
impl RxStandbyStR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RxStandbySt> {
        match self.bits {
            0 => Some(RxStandbySt::B0),
            1 => Some(RxStandbySt::B1),
            _ => None,
        }
    }
    #[doc = "Standby Always high during P1/P2/L1SS state."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RxStandbySt::B0
    }
    #[doc = "Standby Always high during P1/P2/L1SS state."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RxStandbySt::B1
    }
}
#[doc = "PIPE interface data bus width Reports the width of the data bus that the PHY is configured for:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataBusWidth {
    #[doc = "0: 16-bit mode 0thers: reserved When bypass_codec is high, the interface is 20-bit and these two bits report a value of 2'b01."]
    B0 = 0,
    #[doc = "1: 16-bit mode 0thers: reserved When bypass_codec is high, the interface is 20-bit and these two bits report a value of 2'b01."]
    B1 = 1,
}
impl From<DataBusWidth> for u8 {
    #[inline(always)]
    fn from(variant: DataBusWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DataBusWidth {
    type Ux = u8;
}
#[doc = "Field `DATA_BUS_WIDTH` reader - PIPE interface data bus width Reports the width of the data bus that the PHY is configured for:"]
pub type DataBusWidthR = crate::FieldReader<DataBusWidth>;
impl DataBusWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DataBusWidth> {
        match self.bits {
            0 => Some(DataBusWidth::B0),
            1 => Some(DataBusWidth::B1),
            _ => None,
        }
    }
    #[doc = "16-bit mode 0thers: reserved When bypass_codec is high, the interface is 20-bit and these two bits report a value of 2'b01."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DataBusWidth::B0
    }
    #[doc = "16-bit mode 0thers: reserved When bypass_codec is high, the interface is 20-bit and these two bits report a value of 2'b01."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DataBusWidth::B1
    }
}
#[doc = "Field `PHY_ST` reader - PIPE phy status It indicates completion of several PHY functions including power management state transition and receiver detection. When this signal transitions during entry and exit from any PHY state where PCLK is not provided, then the signaling is asynchronous. When phy power up, \"0\" state can indicates pll locked"]
pub type PhyStR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - PIPE phy de-emphasis status Transmitter de-emphasis selection, it combined by tx_deemphasis_ext,tx_deemphasis."]
    #[inline(always)]
    pub fn tx_deemphasis(&self) -> TxDeemphasisR {
        TxDeemphasisR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - RX standby status Indicates PHY's RxStandby state"]
    #[inline(always)]
    pub fn rx_standby_st(&self) -> RxStandbyStR {
        RxStandbyStR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - PIPE interface data bus width Reports the width of the data bus that the PHY is configured for:"]
    #[inline(always)]
    pub fn data_bus_width(&self) -> DataBusWidthR {
        DataBusWidthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - PIPE phy status It indicates completion of several PHY functions including power management state transition and receiver detection. When this signal transitions during entry and exit from any PHY state where PCLK is not provided, then the signaling is asynchronous. When phy power up, \"0\" state can indicates pll locked"]
    #[inline(always)]
    pub fn phy_st(&self) -> PhyStR {
        PhyStR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Side band status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_side_band_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientSideBandStatusSpec;
impl crate::RegisterSpec for PcieClientSideBandStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_side_band_status::R`](R) reader structure"]
impl crate::Readable for PcieClientSideBandStatusSpec {}
#[doc = "`reset()` method sets PCIE_CLIENT_SIDE_BAND_STATUS to value 0x11f1"]
impl crate::Resettable for PcieClientSideBandStatusSpec {
    const RESET_VALUE: u32 = 0x11f1;
}

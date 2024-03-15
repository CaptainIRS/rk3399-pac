#[doc = "Register `USB3_GDBGLTSSM` reader"]
pub type R = crate::R<Usb3GdbgltssmSpec>;
#[doc = "Register `USB3_GDBGLTSSM` writer"]
pub type W = crate::W<Usb3GdbgltssmSpec>;
#[doc = "Field `TXONESZEROS` reader - TXONESZEROS Reflect status of Pipe interface."]
pub type TxoneszerosR = crate::BitReader;
#[doc = "Field `RXTERMINATION` reader - RXTERMINATION Reflect status of Pipe interface."]
pub type RxterminationR = crate::BitReader;
#[doc = "Field `TXSWING` reader - TXSWING Reflect status of Pipe interface."]
pub type TxswingR = crate::BitReader;
#[doc = "LTSSM Clock State In multi-port host configuration, the port number is defined by Port-Select\\[3:0\\]
field in the GDBGFIFOSPACE register. Note:GDBGLTSSM register is not applicable for USB 2.0-only mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ltdbclkstate {
    #[doc = "0: CLK_WAIT2 (Wait for Phy_Status (P3 exit request))"]
    B000 = 0,
    #[doc = "1: CLK_WAIT2 (Wait for Phy_Status (P3 exit request))"]
    B001 = 1,
    #[doc = "2: CLK_WAIT2 (Wait for Phy_Status (P3 exit request))"]
    B010 = 2,
    #[doc = "3: CLK_WAIT2 (Wait for Phy_Status (P3 exit request))"]
    B011 = 3,
    #[doc = "4: CLK_WAIT2 (Wait for Phy_Status (P3 exit request))"]
    B100 = 4,
    #[doc = "5: CLK_WAIT2 (Wait for Phy_Status (P3 exit request))"]
    B101 = 5,
}
impl From<Ltdbclkstate> for u8 {
    #[inline(always)]
    fn from(variant: Ltdbclkstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ltdbclkstate {
    type Ux = u8;
}
#[doc = "Field `LTDBCLKSTATE` reader - LTSSM Clock State In multi-port host configuration, the port number is defined by Port-Select\\[3:0\\]
field in the GDBGFIFOSPACE register. Note:GDBGLTSSM register is not applicable for USB 2.0-only mode."]
pub type LtdbclkstateR = crate::FieldReader<Ltdbclkstate>;
impl LtdbclkstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ltdbclkstate> {
        match self.bits {
            0 => Some(Ltdbclkstate::B000),
            1 => Some(Ltdbclkstate::B001),
            2 => Some(Ltdbclkstate::B010),
            3 => Some(Ltdbclkstate::B011),
            4 => Some(Ltdbclkstate::B100),
            5 => Some(Ltdbclkstate::B101),
            _ => None,
        }
    }
    #[doc = "CLK_WAIT2 (Wait for Phy_Status (P3 exit request))"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Ltdbclkstate::B000
    }
    #[doc = "CLK_WAIT2 (Wait for Phy_Status (P3 exit request))"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Ltdbclkstate::B001
    }
    #[doc = "CLK_WAIT2 (Wait for Phy_Status (P3 exit request))"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Ltdbclkstate::B010
    }
    #[doc = "CLK_WAIT2 (Wait for Phy_Status (P3 exit request))"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Ltdbclkstate::B011
    }
    #[doc = "CLK_WAIT2 (Wait for Phy_Status (P3 exit request))"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Ltdbclkstate::B100
    }
    #[doc = "CLK_WAIT2 (Wait for Phy_Status (P3 exit request))"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Ltdbclkstate::B101
    }
}
#[doc = "Field `TXDEEMPHASIS` reader - TXDEEMPHASIS Reflect status of Pipe interface."]
pub type TxdeemphasisR = crate::FieldReader;
#[doc = "Field `RXEQTRAIN` reader - RXEQTRAIN Reflect status of Pipe interface."]
pub type RxeqtrainR = crate::BitReader;
#[doc = "Field `POWERDOWN` reader - POWERDOWN Reflect status of Pipe interface."]
pub type PowerdownR = crate::FieldReader;
#[doc = "LTSSM PHY command State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ltdbphycmdstate {
    #[doc = "0: PHY_PWR_B (Wait for Phy_Status(Power state change request))"]
    B000 = 0,
    #[doc = "1: PHY_PWR_B (Wait for Phy_Status(Power state change request))"]
    B001 = 1,
    #[doc = "2: PHY_PWR_B (Wait for Phy_Status(Power state change request))"]
    B010 = 2,
    #[doc = "3: PHY_PWR_B (Wait for Phy_Status(Power state change request))"]
    B011 = 3,
    #[doc = "4: PHY_PWR_B (Wait for Phy_Status(Power state change request))"]
    B100 = 4,
    #[doc = "5: PHY_PWR_B (Wait for Phy_Status(Power state change request))"]
    B101 = 5,
}
impl From<Ltdbphycmdstate> for u8 {
    #[inline(always)]
    fn from(variant: Ltdbphycmdstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ltdbphycmdstate {
    type Ux = u8;
}
#[doc = "Field `LTDBPHYCMDSTATE` reader - LTSSM PHY command State"]
pub type LtdbphycmdstateR = crate::FieldReader<Ltdbphycmdstate>;
impl LtdbphycmdstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ltdbphycmdstate> {
        match self.bits {
            0 => Some(Ltdbphycmdstate::B000),
            1 => Some(Ltdbphycmdstate::B001),
            2 => Some(Ltdbphycmdstate::B010),
            3 => Some(Ltdbphycmdstate::B011),
            4 => Some(Ltdbphycmdstate::B100),
            5 => Some(Ltdbphycmdstate::B101),
            _ => None,
        }
    }
    #[doc = "PHY_PWR_B (Wait for Phy_Status(Power state change request))"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Ltdbphycmdstate::B000
    }
    #[doc = "PHY_PWR_B (Wait for Phy_Status(Power state change request))"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Ltdbphycmdstate::B001
    }
    #[doc = "PHY_PWR_B (Wait for Phy_Status(Power state change request))"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Ltdbphycmdstate::B010
    }
    #[doc = "PHY_PWR_B (Wait for Phy_Status(Power state change request))"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Ltdbphycmdstate::B011
    }
    #[doc = "PHY_PWR_B (Wait for Phy_Status(Power state change request))"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Ltdbphycmdstate::B100
    }
    #[doc = "PHY_PWR_B (Wait for Phy_Status(Power state change request))"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Ltdbphycmdstate::B101
    }
}
#[doc = "Field `TXDETRXLOOPBACK` reader - Tx Detect Rx/Loopback Reflect status of Pipe interface."]
pub type TxdetrxloopbackR = crate::BitReader;
#[doc = "Field `RXPOLARITY` reader - RXPOLARITY Reflect status of Pipe interface."]
pub type RxpolarityR = crate::BitReader;
#[doc = "Field `TXELECLDLE` reader - TXELECLDLE Reflect status of Pipe interface."]
pub type TxelecldleR = crate::BitReader;
#[doc = "Field `ELASTICBUFFERMODE` reader - ELASTICBUFFERMODE Reflect status of Pipe interface."]
pub type ElasticbuffermodeR = crate::BitReader;
#[doc = "Field `LTDBSUBSTATE` reader - LTDB Sub-State LTDB Sub-State"]
pub type LtdbsubstateR = crate::FieldReader;
#[doc = "Field `LTDBLINKSTATE` reader - LTDB Link State LTDB Link State"]
pub type LtdblinkstateR = crate::FieldReader;
#[doc = "Field `LTDBTIMEOUT` reader - LTDB Timeout LTDB Timeout"]
pub type LtdbtimeoutR = crate::BitReader;
#[doc = "Field `LTDBTIMEOUT` writer - LTDB Timeout LTDB Timeout"]
pub type LtdbtimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXELECIDLE` reader - RxElecidle Reflect status of Pipe interface."]
pub type RxelecidleR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TXONESZEROS Reflect status of Pipe interface."]
    #[inline(always)]
    pub fn txoneszeros(&self) -> TxoneszerosR {
        TxoneszerosR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXTERMINATION Reflect status of Pipe interface."]
    #[inline(always)]
    pub fn rxtermination(&self) -> RxterminationR {
        RxterminationR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXSWING Reflect status of Pipe interface."]
    #[inline(always)]
    pub fn txswing(&self) -> TxswingR {
        TxswingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - LTSSM Clock State In multi-port host configuration, the port number is defined by Port-Select\\[3:0\\]
field in the GDBGFIFOSPACE register. Note:GDBGLTSSM register is not applicable for USB 2.0-only mode."]
    #[inline(always)]
    pub fn ltdbclkstate(&self) -> LtdbclkstateR {
        LtdbclkstateR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - TXDEEMPHASIS Reflect status of Pipe interface."]
    #[inline(always)]
    pub fn txdeemphasis(&self) -> TxdeemphasisR {
        TxdeemphasisR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - RXEQTRAIN Reflect status of Pipe interface."]
    #[inline(always)]
    pub fn rxeqtrain(&self) -> RxeqtrainR {
        RxeqtrainR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - POWERDOWN Reflect status of Pipe interface."]
    #[inline(always)]
    pub fn powerdown(&self) -> PowerdownR {
        PowerdownR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:13 - LTSSM PHY command State"]
    #[inline(always)]
    pub fn ltdbphycmdstate(&self) -> LtdbphycmdstateR {
        LtdbphycmdstateR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Tx Detect Rx/Loopback Reflect status of Pipe interface."]
    #[inline(always)]
    pub fn txdetrxloopback(&self) -> TxdetrxloopbackR {
        TxdetrxloopbackR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXPOLARITY Reflect status of Pipe interface."]
    #[inline(always)]
    pub fn rxpolarity(&self) -> RxpolarityR {
        RxpolarityR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TXELECLDLE Reflect status of Pipe interface."]
    #[inline(always)]
    pub fn txelecldle(&self) -> TxelecldleR {
        TxelecldleR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ELASTICBUFFERMODE Reflect status of Pipe interface."]
    #[inline(always)]
    pub fn elasticbuffermode(&self) -> ElasticbuffermodeR {
        ElasticbuffermodeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - LTDB Sub-State LTDB Sub-State"]
    #[inline(always)]
    pub fn ltdbsubstate(&self) -> LtdbsubstateR {
        LtdbsubstateR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - LTDB Link State LTDB Link State"]
    #[inline(always)]
    pub fn ltdblinkstate(&self) -> LtdblinkstateR {
        LtdblinkstateR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - LTDB Timeout LTDB Timeout"]
    #[inline(always)]
    pub fn ltdbtimeout(&self) -> LtdbtimeoutR {
        LtdbtimeoutR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - RxElecidle Reflect status of Pipe interface."]
    #[inline(always)]
    pub fn rxelecidle(&self) -> RxelecidleR {
        RxelecidleR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - LTDB Timeout LTDB Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn ltdbtimeout(&mut self) -> LtdbtimeoutW<Usb3GdbgltssmSpec> {
        LtdbtimeoutW::new(self, 26)
    }
}
#[doc = "Global Debug LTSSM Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_gdbgltssm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_gdbgltssm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GdbgltssmSpec;
impl crate::RegisterSpec for Usb3GdbgltssmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_gdbgltssm::R`](R) reader structure"]
impl crate::Readable for Usb3GdbgltssmSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_gdbgltssm::W`](W) writer structure"]
impl crate::Writable for Usb3GdbgltssmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GDBGLTSSM to value 0x4101_0440"]
impl crate::Resettable for Usb3GdbgltssmSpec {
    const RESET_VALUE: u32 = 0x4101_0440;
}

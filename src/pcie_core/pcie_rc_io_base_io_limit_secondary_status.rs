#[doc = "Register `PCIE_RC_IO_BASE_IO_LIMIT_SECONDARY_STATUS` reader"]
pub type R = crate::R<PcieRcIoBaseIoLimitSecondaryStatusSpec>;
#[doc = "Register `PCIE_RC_IO_BASE_IO_LIMIT_SECONDARY_STATUS` writer"]
pub type W = crate::W<PcieRcIoBaseIoLimitSecondaryStatusSpec>;
#[doc = "Field `IOBS1` reader - Type1 cfg IO bar size \\[IOBS1\\]
value set in Type1 cfg IO bar size(bit 20 of RC BAR CONFIG register). If type1 cfg IO bar enable bit(bit 19 in RC BAR CONFIG register) is not set, then this field will be hard coded to 0."]
pub type Iobs1R = crate::BitReader;
#[doc = "Field `R1` reader - Reserved \\[R1\\]
Reserved"]
pub type R1R = crate::FieldReader;
#[doc = "Field `IBR` reader - IO Base Register \\[IBR\\]
This field can be read and written from the local management bus if IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub type IbrR = crate::FieldReader;
#[doc = "Field `IOBS2` reader - Type1 cfg IO bar size \\[IOBS2\\]
value set in Type1 cfg IO bar size(bit 20 of RC BAR CONFIG register).If type1 cfg IObar enable bit(bit 19 in RC BAR CONFIG register) is not set, then this field will be hard coded to 0."]
pub type Iobs2R = crate::BitReader;
#[doc = "Field `R2` reader - Reserved \\[R2\\]
Reserved"]
pub type R2R = crate::FieldReader;
#[doc = "Field `ILR` reader - IO Limit Register \\[ILR\\]
This field can be read and written from the local management bus if IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
pub type IlrR = crate::FieldReader;
#[doc = "Field `R3` reader - Reserved \\[R3\\]
Reserved"]
pub type R3R = crate::FieldReader;
#[doc = "Field `MPE` reader - Master Data Parity Error \\[MPE\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write. Note that this bit can be set only when the Parity Error Response Enable bit is set in the Bridge Control Register"]
pub type MpeR = crate::BitReader;
#[doc = "Field `MPE` writer - Master Data Parity Error \\[MPE\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write. Note that this bit can be set only when the Parity Error Response Enable bit is set in the Bridge Control Register"]
pub type MpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
Reserved"]
pub type R4R = crate::FieldReader;
#[doc = "Field `STA` reader - Signaled Target Abort \\[STA\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type StaR = crate::BitReader;
#[doc = "Field `STA` writer - Signaled Target Abort \\[STA\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type StaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RTA` reader - Recieved Target Abort \\[RTA\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RtaR = crate::BitReader;
#[doc = "Field `RTA` writer - Recieved Target Abort \\[RTA\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RtaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RMA` reader - Received Master Abort \\[RMA\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RmaR = crate::BitReader;
#[doc = "Field `RMA` writer - Received Master Abort \\[RMA\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RmaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RSE` reader - Received System Error \\[RSE\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RseR = crate::BitReader;
#[doc = "Field `RSE` writer - Received System Error \\[RSE\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RseW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DPE` reader - Detected Parity Error \\[DPE\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type DpeR = crate::BitReader;
#[doc = "Field `DPE` writer - Detected Parity Error \\[DPE\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type DpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Type1 cfg IO bar size \\[IOBS1\\]
value set in Type1 cfg IO bar size(bit 20 of RC BAR CONFIG register). If type1 cfg IO bar enable bit(bit 19 in RC BAR CONFIG register) is not set, then this field will be hard coded to 0."]
    #[inline(always)]
    pub fn iobs1(&self) -> Iobs1R {
        Iobs1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved \\[R1\\]
Reserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - IO Base Register \\[IBR\\]
This field can be read and written from the local management bus if IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub fn ibr(&self) -> IbrR {
        IbrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Type1 cfg IO bar size \\[IOBS2\\]
value set in Type1 cfg IO bar size(bit 20 of RC BAR CONFIG register).If type1 cfg IObar enable bit(bit 19 in RC BAR CONFIG register) is not set, then this field will be hard coded to 0."]
    #[inline(always)]
    pub fn iobs2(&self) -> Iobs2R {
        Iobs2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Reserved \\[R2\\]
Reserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:15 - IO Limit Register \\[ILR\\]
This field can be read and written from the local management bus if IO BAR is enabled in the Root Complex BAR configuration register, else it is hardwired to zero. Its value is not used within the core."]
    #[inline(always)]
    pub fn ilr(&self) -> IlrR {
        IlrR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Reserved \\[R3\\]
Reserved"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Master Data Parity Error \\[MPE\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write. Note that this bit can be set only when the Parity Error Response Enable bit is set in the Bridge Control Register"]
    #[inline(always)]
    pub fn mpe(&self) -> MpeR {
        MpeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Reserved \\[R4\\]
Reserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Signaled Target Abort \\[STA\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn sta(&self) -> StaR {
        StaR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Recieved Target Abort \\[RTA\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn rta(&self) -> RtaR {
        RtaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Received Master Abort \\[RMA\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn rma(&self) -> RmaR {
        RmaR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Received System Error \\[RSE\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn rse(&self) -> RseR {
        RseR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Detected Parity Error \\[DPE\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn dpe(&self) -> DpeR {
        DpeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Master Data Parity Error \\[MPE\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write. Note that this bit can be set only when the Parity Error Response Enable bit is set in the Bridge Control Register"]
    #[inline(always)]
    #[must_use]
    pub fn mpe(&mut self) -> MpeW<PcieRcIoBaseIoLimitSecondaryStatusSpec> {
        MpeW::new(self, 24)
    }
    #[doc = "Bit 27 - Signaled Target Abort \\[STA\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> StaW<PcieRcIoBaseIoLimitSecondaryStatusSpec> {
        StaW::new(self, 27)
    }
    #[doc = "Bit 28 - Recieved Target Abort \\[RTA\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn rta(&mut self) -> RtaW<PcieRcIoBaseIoLimitSecondaryStatusSpec> {
        RtaW::new(self, 28)
    }
    #[doc = "Bit 29 - Received Master Abort \\[RMA\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn rma(&mut self) -> RmaW<PcieRcIoBaseIoLimitSecondaryStatusSpec> {
        RmaW::new(self, 29)
    }
    #[doc = "Bit 30 - Received System Error \\[RSE\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RseW<PcieRcIoBaseIoLimitSecondaryStatusSpec> {
        RseW::new(self, 30)
    }
    #[doc = "Bit 31 - Detected Parity Error \\[DPE\\]
The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn dpe(&mut self) -> DpeW<PcieRcIoBaseIoLimitSecondaryStatusSpec> {
        DpeW::new(self, 31)
    }
}
#[doc = "IO Base, IO Limit, Secondary Status Register The core does not set this bit by itself. This bit can be cleared by writing a 1 into this bit position from the local management APB bus. This field can be forced to 1 or 0 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_io_base_io_limit_secondary_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_io_base_io_limit_secondary_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcIoBaseIoLimitSecondaryStatusSpec;
impl crate::RegisterSpec for PcieRcIoBaseIoLimitSecondaryStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_io_base_io_limit_secondary_status::R`](R) reader structure"]
impl crate::Readable for PcieRcIoBaseIoLimitSecondaryStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_io_base_io_limit_secondary_status::W`](W) writer structure"]
impl crate::Writable for PcieRcIoBaseIoLimitSecondaryStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf900_0000;
}
#[doc = "`reset()` method sets PCIE_RC_IO_BASE_IO_LIMIT_SECONDARY_STATUS to value 0"]
impl crate::Resettable for PcieRcIoBaseIoLimitSecondaryStatusSpec {
    const RESET_VALUE: u32 = 0;
}

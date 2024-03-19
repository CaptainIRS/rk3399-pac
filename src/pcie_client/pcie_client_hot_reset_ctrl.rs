#[doc = "Register `PCIE_CLIENT_HOT_RESET_CTRL` reader"]
pub type R = crate::R<PcieClientHotResetCtrlSpec>;
#[doc = "Register `PCIE_CLIENT_HOT_RESET_CTRL` writer"]
pub type W = crate::W<PcieClientHotResetCtrlSpec>;
#[doc = "Assert hot reset to remote device\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HotResetIn {
    #[doc = "0: de-assert"]
    B0 = 0,
    #[doc = "1: assert When this input is asserted in the RC mode, the core initiates a Hot Reset sequence on the PCIe link. This signal should be driven synchronous to the CORE_CLK domain. The controller will keep the PCIe link in hot reset till the time this signal is driven asserted. When de-asserted, controller will bring the PCIe link out of hot reset and initiate link training"]
    B1 = 1,
}
impl From<HotResetIn> for bool {
    #[inline(always)]
    fn from(variant: HotResetIn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOT_RESET_IN` reader - Assert hot reset to remote device"]
pub type HotResetInR = crate::BitReader<HotResetIn>;
impl HotResetInR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HotResetIn {
        match self.bits {
            false => HotResetIn::B0,
            true => HotResetIn::B1,
        }
    }
    #[doc = "de-assert"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HotResetIn::B0
    }
    #[doc = "assert When this input is asserted in the RC mode, the core initiates a Hot Reset sequence on the PCIe link. This signal should be driven synchronous to the CORE_CLK domain. The controller will keep the PCIe link in hot reset till the time this signal is driven asserted. When de-asserted, controller will bring the PCIe link out of hot reset and initiate link training"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HotResetIn::B1
    }
}
#[doc = "Field `HOT_RESET_IN` writer - Assert hot reset to remote device"]
pub type HotResetInW<'a, REG> = crate::BitWriter<'a, REG, HotResetIn>;
impl<'a, REG> HotResetInW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "de-assert"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HotResetIn::B0)
    }
    #[doc = "assert When this input is asserted in the RC mode, the core initiates a Hot Reset sequence on the PCIe link. This signal should be driven synchronous to the CORE_CLK domain. The controller will keep the PCIe link in hot reset till the time this signal is driven asserted. When de-asserted, controller will bring the PCIe link out of hot reset and initiate link training"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HotResetIn::B1)
    }
}
#[doc = "Mask link down reset client logic\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkDownRstCltMask {
    #[doc = "0: disable link down reset client logic"]
    B0 = 0,
    #[doc = "1: enable link down reset client logic"]
    B1 = 1,
}
impl From<LinkDownRstCltMask> for bool {
    #[inline(always)]
    fn from(variant: LinkDownRstCltMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINK_DOWN_RST_CLT_MASK` reader - Mask link down reset client logic"]
pub type LinkDownRstCltMaskR = crate::BitReader<LinkDownRstCltMask>;
impl LinkDownRstCltMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinkDownRstCltMask {
        match self.bits {
            false => LinkDownRstCltMask::B0,
            true => LinkDownRstCltMask::B1,
        }
    }
    #[doc = "disable link down reset client logic"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == LinkDownRstCltMask::B0
    }
    #[doc = "enable link down reset client logic"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == LinkDownRstCltMask::B1
    }
}
#[doc = "Field `LINK_DOWN_RST_CLT_MASK` writer - Mask link down reset client logic"]
pub type LinkDownRstCltMaskW<'a, REG> = crate::BitWriter<'a, REG, LinkDownRstCltMask>;
impl<'a, REG> LinkDownRstCltMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable link down reset client logic"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(LinkDownRstCltMask::B0)
    }
    #[doc = "enable link down reset client logic"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(LinkDownRstCltMask::B1)
    }
}
#[doc = "Write mask\n\nFor each served bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WriteMask {
    #[doc = "0: write mask"]
    B0 = 0,
    #[doc = "1: write enable"]
    B1 = 1,
}
impl From<WriteMask> for u8 {
    #[inline(always)]
    fn from(variant: WriteMask) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WriteMask {
    type Ux = u8;
}
#[doc = "Field `WRITE_MASK` writer - Write mask\n\nFor each served bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2, WriteMask>;
impl<'a, REG> WriteMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "write mask"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B0)
    }
    #[doc = "write enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMask::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Assert hot reset to remote device"]
    #[inline(always)]
    pub fn hot_reset_in(&self) -> HotResetInR {
        HotResetInR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask link down reset client logic"]
    #[inline(always)]
    pub fn link_down_rst_clt_mask(&self) -> LinkDownRstCltMaskR {
        LinkDownRstCltMaskR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Assert hot reset to remote device"]
    #[inline(always)]
    #[must_use]
    pub fn hot_reset_in(&mut self) -> HotResetInW<PcieClientHotResetCtrlSpec> {
        HotResetInW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask link down reset client logic"]
    #[inline(always)]
    #[must_use]
    pub fn link_down_rst_clt_mask(&mut self) -> LinkDownRstCltMaskW<PcieClientHotResetCtrlSpec> {
        LinkDownRstCltMaskW::new(self, 1)
    }
    #[doc = "Bits 16:17 - Write mask\n\nFor each served bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<PcieClientHotResetCtrlSpec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Hot reset control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_client_hot_reset_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_client_hot_reset_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieClientHotResetCtrlSpec;
impl crate::RegisterSpec for PcieClientHotResetCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_client_hot_reset_ctrl::R`](R) reader structure"]
impl crate::Readable for PcieClientHotResetCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_client_hot_reset_ctrl::W`](W) writer structure"]
impl crate::Writable for PcieClientHotResetCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_CLIENT_HOT_RESET_CTRL to value 0"]
impl crate::Resettable for PcieClientHotResetCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

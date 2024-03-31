#[doc = "Register `LEGACY_INT_CTRL` reader"]
pub type R = crate::R<LegacyIntCtrlSpec>;
#[doc = "Register `LEGACY_INT_CTRL` writer"]
pub type W = crate::W<LegacyIntCtrlSpec>;
#[doc = "Legacy interrupt pending status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntPendSt {
    #[doc = "0: normal"]
    B0 = 0,
    #[doc = "1: pending When using legacy interrupts, this input is used to indicate the interrupt pending status of the Physical Functions. The input i must be set when an interrupt is pending in Function i."]
    B1 = 1,
}
impl From<IntPendSt> for bool {
    #[inline(always)]
    fn from(variant: IntPendSt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_PEND_ST` reader - Legacy interrupt pending status"]
pub type IntPendStR = crate::BitReader<IntPendSt>;
impl IntPendStR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntPendSt {
        match self.bits {
            false => IntPendSt::B0,
            true => IntPendSt::B1,
        }
    }
    #[doc = "normal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntPendSt::B0
    }
    #[doc = "pending When using legacy interrupts, this input is used to indicate the interrupt pending status of the Physical Functions. The input i must be set when an interrupt is pending in Function i."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntPendSt::B1
    }
}
#[doc = "Field `INT_PEND_ST` writer - Legacy interrupt pending status"]
pub type IntPendStW<'a, REG> = crate::BitWriter<'a, REG, IntPendSt>;
impl<'a, REG> IntPendStW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntPendSt::B0)
    }
    #[doc = "pending When using legacy interrupts, this input is used to indicate the interrupt pending status of the Physical Functions. The input i must be set when an interrupt is pending in Function i."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntPendSt::B1)
    }
}
#[doc = "Legacy interrupt input\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntIn {
    #[doc = "0: de-assert"]
    B0 = 0,
    #[doc = "1: assert When the core is configured as EP, this input is used by the client application to signal an interrupt from any of its PCI Functions to the RC using the Legacy PCI Express Interrupt Delivery mechanism of PCI Express. This input corresponds to INTA of the PCI bus. Asserting this signal causes the core to send out an Assert_INTx message, and de-asserting this signal causes the core to transmit a Deassert_INTx message."]
    B1 = 1,
}
impl From<IntIn> for bool {
    #[inline(always)]
    fn from(variant: IntIn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_IN` reader - Legacy interrupt input"]
pub type IntInR = crate::BitReader<IntIn>;
impl IntInR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntIn {
        match self.bits {
            false => IntIn::B0,
            true => IntIn::B1,
        }
    }
    #[doc = "de-assert"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IntIn::B0
    }
    #[doc = "assert When the core is configured as EP, this input is used by the client application to signal an interrupt from any of its PCI Functions to the RC using the Legacy PCI Express Interrupt Delivery mechanism of PCI Express. This input corresponds to INTA of the PCI bus. Asserting this signal causes the core to send out an Assert_INTx message, and de-asserting this signal causes the core to transmit a Deassert_INTx message."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IntIn::B1
    }
}
#[doc = "Field `INT_IN` writer - Legacy interrupt input"]
pub type IntInW<'a, REG> = crate::BitWriter<'a, REG, IntIn>;
impl<'a, REG> IntInW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "de-assert"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IntIn::B0)
    }
    #[doc = "assert When the core is configured as EP, this input is used by the client application to signal an interrupt from any of its PCI Functions to the RC using the Legacy PCI Express Interrupt Delivery mechanism of PCI Express. This input corresponds to INTA of the PCI bus. Asserting this signal causes the core to send out an Assert_INTx message, and de-asserting this signal causes the core to transmit a Deassert_INTx message."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IntIn::B1)
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
    #[doc = "Bit 0 - Legacy interrupt pending status"]
    #[inline(always)]
    pub fn int_pend_st(&self) -> IntPendStR {
        IntPendStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Legacy interrupt input"]
    #[inline(always)]
    pub fn int_in(&self) -> IntInR {
        IntInR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Legacy interrupt pending status"]
    #[inline(always)]
    #[must_use]
    pub fn int_pend_st(&mut self) -> IntPendStW<LegacyIntCtrlSpec> {
        IntPendStW::new(self, 0)
    }
    #[doc = "Bit 1 - Legacy interrupt input"]
    #[inline(always)]
    #[must_use]
    pub fn int_in(&mut self) -> IntInW<LegacyIntCtrlSpec> {
        IntInW::new(self, 1)
    }
    #[doc = "Bits 16:17 - Write mask\n\nFor each served bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<LegacyIntCtrlSpec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Legacy interrupt control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`legacy_int_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`legacy_int_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LegacyIntCtrlSpec;
impl crate::RegisterSpec for LegacyIntCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`legacy_int_ctrl::R`](R) reader structure"]
impl crate::Readable for LegacyIntCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`legacy_int_ctrl::W`](W) writer structure"]
impl crate::Writable for LegacyIntCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LEGACY_INT_CTRL to value 0"]
impl crate::Resettable for LegacyIntCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

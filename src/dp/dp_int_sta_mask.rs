#[doc = "Register `DP_INT_STA_MASK` reader"]
pub type R = crate::R<DpIntStaMaskSpec>;
#[doc = "Register `DP_INT_STA_MASK` writer"]
pub type W = crate::W<DpIntStaMaskSpec>;
#[doc = "Each bit corresponds to the same bit in \n\nDisplayPort Interrupt Status Register \n\n(DP_INT_STA).\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DpIntStaMask {
    #[doc = "1: Enable interrupt."]
    B1 = 1,
    #[doc = "0: Mask interrupt."]
    B0 = 0,
}
impl From<DpIntStaMask> for u8 {
    #[inline(always)]
    fn from(variant: DpIntStaMask) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DpIntStaMask {
    type Ux = u8;
}
#[doc = "Field `DP_INT_STA_MASK` reader - Each bit corresponds to the same bit in \n\nDisplayPort Interrupt Status Register \n\n(DP_INT_STA)."]
pub type DpIntStaMaskR = crate::FieldReader<DpIntStaMask>;
impl DpIntStaMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DpIntStaMask> {
        match self.bits {
            1 => Some(DpIntStaMask::B1),
            0 => Some(DpIntStaMask::B0),
            _ => None,
        }
    }
    #[doc = "Enable interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DpIntStaMask::B1
    }
    #[doc = "Mask interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DpIntStaMask::B0
    }
}
#[doc = "Field `DP_INT_STA_MASK` writer - Each bit corresponds to the same bit in \n\nDisplayPort Interrupt Status Register \n\n(DP_INT_STA)."]
pub type DpIntStaMaskW<'a, REG> = crate::FieldWriter<'a, REG, 7, DpIntStaMask>;
impl<'a, REG> DpIntStaMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DpIntStaMask::B1)
    }
    #[doc = "Mask interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DpIntStaMask::B0)
    }
}
impl R {
    #[doc = "Bits 0:6 - Each bit corresponds to the same bit in \n\nDisplayPort Interrupt Status Register \n\n(DP_INT_STA)."]
    #[inline(always)]
    pub fn dp_int_sta_mask(&self) -> DpIntStaMaskR {
        DpIntStaMaskR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Each bit corresponds to the same bit in \n\nDisplayPort Interrupt Status Register \n\n(DP_INT_STA)."]
    #[inline(always)]
    #[must_use]
    pub fn dp_int_sta_mask(&mut self) -> DpIntStaMaskW<DpIntStaMaskSpec> {
        DpIntStaMaskW::new(self, 0)
    }
}
#[doc = "DisplayPort Interrupt enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_int_sta_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_int_sta_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpIntStaMaskSpec;
impl crate::RegisterSpec for DpIntStaMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_int_sta_mask::R`](R) reader structure"]
impl crate::Readable for DpIntStaMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_int_sta_mask::W`](W) writer structure"]
impl crate::Writable for DpIntStaMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DP_INT_STA_MASK to value 0"]
impl crate::Resettable for DpIntStaMaskSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `COMMON_INT_MASK_4` reader"]
pub type R = crate::R<CommonIntMask4Spec>;
#[doc = "Register `COMMON_INT_MASK_4` writer"]
pub type W = crate::W<CommonIntMask4Spec>;
#[doc = "Each bit corresponds to the same bit in \n\nCommon Interrupt Status Register 3.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CommonIntMask4 {
    #[doc = "0: Mask interrupt"]
    B0 = 0,
    #[doc = "1: Enable interrupt"]
    B1 = 1,
}
impl From<CommonIntMask4> for u8 {
    #[inline(always)]
    fn from(variant: CommonIntMask4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CommonIntMask4 {
    type Ux = u8;
}
#[doc = "Field `COMMON_INT_MASK_4` reader - Each bit corresponds to the same bit in \n\nCommon Interrupt Status Register 3."]
pub type CommonIntMask4R = crate::FieldReader<CommonIntMask4>;
impl CommonIntMask4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CommonIntMask4> {
        match self.bits {
            0 => Some(CommonIntMask4::B0),
            1 => Some(CommonIntMask4::B1),
            _ => None,
        }
    }
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CommonIntMask4::B0
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CommonIntMask4::B1
    }
}
#[doc = "Field `COMMON_INT_MASK_4` writer - Each bit corresponds to the same bit in \n\nCommon Interrupt Status Register 3."]
pub type CommonIntMask4W<'a, REG> = crate::FieldWriter<'a, REG, 3, CommonIntMask4>;
impl<'a, REG> CommonIntMask4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mask interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CommonIntMask4::B0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CommonIntMask4::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Each bit corresponds to the same bit in \n\nCommon Interrupt Status Register 3."]
    #[inline(always)]
    pub fn common_int_mask_4(&self) -> CommonIntMask4R {
        CommonIntMask4R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Each bit corresponds to the same bit in \n\nCommon Interrupt Status Register 3."]
    #[inline(always)]
    #[must_use]
    pub fn common_int_mask_4(&mut self) -> CommonIntMask4W<CommonIntMask4Spec> {
        CommonIntMask4W::new(self, 0)
    }
}
#[doc = "Common Interrupt Mask Register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`common_int_mask_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`common_int_mask_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommonIntMask4Spec;
impl crate::RegisterSpec for CommonIntMask4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`common_int_mask_4::R`](R) reader structure"]
impl crate::Readable for CommonIntMask4Spec {}
#[doc = "`write(|w| ..)` method takes [`common_int_mask_4::W`](W) writer structure"]
impl crate::Writable for CommonIntMask4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets COMMON_INT_MASK_4 to value 0"]
impl crate::Resettable for CommonIntMask4Spec {
    const RESET_VALUE: u32 = 0;
}

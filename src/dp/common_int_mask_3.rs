#[doc = "Register `COMMON_INT_MASK_3` reader"]
pub type R = crate::R<CommonIntMask3Spec>;
#[doc = "Register `COMMON_INT_MASK_3` writer"]
pub type W = crate::W<CommonIntMask3Spec>;
#[doc = "Each bit corresponds to the same bit in Common Interrupt Status Register 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CommonIntMask3 {
    #[doc = "0: Enable interrupt"]
    B0 = 0,
    #[doc = "1: Enable interrupt"]
    B1 = 1,
}
impl From<CommonIntMask3> for u8 {
    #[inline(always)]
    fn from(variant: CommonIntMask3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CommonIntMask3 {
    type Ux = u8;
}
#[doc = "Field `COMMON_INT_MASK_3` reader - Each bit corresponds to the same bit in Common Interrupt Status Register 3."]
pub type CommonIntMask3R = crate::FieldReader<CommonIntMask3>;
impl CommonIntMask3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CommonIntMask3> {
        match self.bits {
            0 => Some(CommonIntMask3::B0),
            1 => Some(CommonIntMask3::B1),
            _ => None,
        }
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CommonIntMask3::B0
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CommonIntMask3::B1
    }
}
#[doc = "Field `COMMON_INT_MASK_3` writer - Each bit corresponds to the same bit in Common Interrupt Status Register 3."]
pub type CommonIntMask3W<'a, REG> = crate::FieldWriter<'a, REG, 4, CommonIntMask3>;
impl<'a, REG> CommonIntMask3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CommonIntMask3::B0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CommonIntMask3::B1)
    }
}
impl R {
    #[doc = "Bits 1:4 - Each bit corresponds to the same bit in Common Interrupt Status Register 3."]
    #[inline(always)]
    pub fn common_int_mask_3(&self) -> CommonIntMask3R {
        CommonIntMask3R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:4 - Each bit corresponds to the same bit in Common Interrupt Status Register 3."]
    #[inline(always)]
    #[must_use]
    pub fn common_int_mask_3(&mut self) -> CommonIntMask3W<CommonIntMask3Spec> {
        CommonIntMask3W::new(self, 1)
    }
}
#[doc = "Common Interrupt Mask Register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`common_int_mask_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`common_int_mask_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommonIntMask3Spec;
impl crate::RegisterSpec for CommonIntMask3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`common_int_mask_3::R`](R) reader structure"]
impl crate::Readable for CommonIntMask3Spec {}
#[doc = "`write(|w| ..)` method takes [`common_int_mask_3::W`](W) writer structure"]
impl crate::Writable for CommonIntMask3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1e;
}
#[doc = "`reset()` method sets COMMON_INT_MASK_3 to value 0"]
impl crate::Resettable for CommonIntMask3Spec {
    const RESET_VALUE: u32 = 0;
}

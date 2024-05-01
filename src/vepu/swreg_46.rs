#[doc = "Register `SWREG_46` reader"]
pub type R = crate::R<Swreg46Spec>;
#[doc = "Register `SWREG_46` writer"]
pub type W = crate::W<Swreg46Spec>;
#[doc = "Field `INTRA_RIGHT_MB_AREA` reader - The right intra macro block's area used in column\n\nThe right intra macro block's area used in column"]
pub type IntraRightMbAreaR = crate::FieldReader;
#[doc = "Field `INTRA_RIGHT_MB_AREA` writer - The right intra macro block's area used in column\n\nThe right intra macro block's area used in column"]
pub type IntraRightMbAreaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INTRA_LEFT_MB_AREA` reader - The left intra macro block's area used in column\n\nThe left intra macro block's area used in column"]
pub type IntraLeftMbAreaR = crate::FieldReader;
#[doc = "Field `INTRA_LEFT_MB_AREA` writer - The left intra macro block's area used in column\n\nThe left intra macro block's area used in column"]
pub type IntraLeftMbAreaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INTRA_DOWN_MB_AREA` reader - The down intra macro block's area used in row\n\nThe bottom intra macro block's area used in row"]
pub type IntraDownMbAreaR = crate::FieldReader;
#[doc = "Field `INTRA_DOWN_MB_AREA` writer - The down intra macro block's area used in row\n\nThe bottom intra macro block's area used in row"]
pub type IntraDownMbAreaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INTRA_UP_MB_AREA` reader - The up intra macro block's area used in row\n\nThe top intra macro block's area used in row"]
pub type IntraUpMbAreaR = crate::FieldReader;
#[doc = "Field `INTRA_UP_MB_AREA` writer - The up intra macro block's area used in row\n\nThe top intra macro block's area used in row"]
pub type IntraUpMbAreaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The right intra macro block's area used in column\n\nThe right intra macro block's area used in column"]
    #[inline(always)]
    pub fn intra_right_mb_area(&self) -> IntraRightMbAreaR {
        IntraRightMbAreaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The left intra macro block's area used in column\n\nThe left intra macro block's area used in column"]
    #[inline(always)]
    pub fn intra_left_mb_area(&self) -> IntraLeftMbAreaR {
        IntraLeftMbAreaR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The down intra macro block's area used in row\n\nThe bottom intra macro block's area used in row"]
    #[inline(always)]
    pub fn intra_down_mb_area(&self) -> IntraDownMbAreaR {
        IntraDownMbAreaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The up intra macro block's area used in row\n\nThe top intra macro block's area used in row"]
    #[inline(always)]
    pub fn intra_up_mb_area(&self) -> IntraUpMbAreaR {
        IntraUpMbAreaR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The right intra macro block's area used in column\n\nThe right intra macro block's area used in column"]
    #[inline(always)]
    #[must_use]
    pub fn intra_right_mb_area(&mut self) -> IntraRightMbAreaW<Swreg46Spec> {
        IntraRightMbAreaW::new(self, 0)
    }
    #[doc = "Bits 8:15 - The left intra macro block's area used in column\n\nThe left intra macro block's area used in column"]
    #[inline(always)]
    #[must_use]
    pub fn intra_left_mb_area(&mut self) -> IntraLeftMbAreaW<Swreg46Spec> {
        IntraLeftMbAreaW::new(self, 8)
    }
    #[doc = "Bits 16:23 - The down intra macro block's area used in row\n\nThe bottom intra macro block's area used in row"]
    #[inline(always)]
    #[must_use]
    pub fn intra_down_mb_area(&mut self) -> IntraDownMbAreaW<Swreg46Spec> {
        IntraDownMbAreaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - The up intra macro block's area used in row\n\nThe top intra macro block's area used in row"]
    #[inline(always)]
    #[must_use]
    pub fn intra_up_mb_area(&mut self) -> IntraUpMbAreaW<Swreg46Spec> {
        IntraUpMbAreaW::new(self, 24)
    }
}
#[doc = "intra macro block sellect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_46::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_46::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg46Spec;
impl crate::RegisterSpec for Swreg46Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_46::R`](R) reader structure"]
impl crate::Readable for Swreg46Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_46::W`](W) writer structure"]
impl crate::Writable for Swreg46Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_46 to value 0"]
impl crate::Resettable for Swreg46Spec {
    const RESET_VALUE: u32 = 0;
}

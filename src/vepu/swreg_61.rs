#[doc = "Register `SWREG_61` reader"]
pub type R = crate::R<Swreg61Spec>;
#[doc = "Register `SWREG_61` writer"]
pub type W = crate::W<Swreg61Spec>;
#[doc = "Field `ROW_LEN_IN_LUMA` reader - the row length of input luminance"]
pub type RowLenInLumaR = crate::FieldReader<u16>;
#[doc = "Field `ROW_LEN_IN_LUMA` writer - the row length of input luminance"]
pub type RowLenInLumaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `OFFSET_IN_LUMA` reader - then offset of input luminance\n\nbyte unit"]
pub type OffsetInLumaR = crate::FieldReader;
#[doc = "Field `OFFSET_IN_LUMA` writer - then offset of input luminance\n\nbyte unit"]
pub type OffsetInLumaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OFFSET_IN_CHROMA` reader - then offset of input chroma\n\nbyte unit"]
pub type OffsetInChromaR = crate::FieldReader;
#[doc = "Field `OFFSET_IN_CHROMA` writer - then offset of input chroma\n\nbyte unit"]
pub type OffsetInChromaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:13 - the row length of input luminance"]
    #[inline(always)]
    pub fn row_len_in_luma(&self) -> RowLenInLumaR {
        RowLenInLumaR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:18 - then offset of input luminance\n\nbyte unit"]
    #[inline(always)]
    pub fn offset_in_luma(&self) -> OffsetInLumaR {
        OffsetInLumaR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - then offset of input chroma\n\nbyte unit"]
    #[inline(always)]
    pub fn offset_in_chroma(&self) -> OffsetInChromaR {
        OffsetInChromaR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - the row length of input luminance"]
    #[inline(always)]
    #[must_use]
    pub fn row_len_in_luma(&mut self) -> RowLenInLumaW<Swreg61Spec> {
        RowLenInLumaW::new(self, 0)
    }
    #[doc = "Bits 16:18 - then offset of input luminance\n\nbyte unit"]
    #[inline(always)]
    #[must_use]
    pub fn offset_in_luma(&mut self) -> OffsetInLumaW<Swreg61Spec> {
        OffsetInLumaW::new(self, 16)
    }
    #[doc = "Bits 20:22 - then offset of input chroma\n\nbyte unit"]
    #[inline(always)]
    #[must_use]
    pub fn offset_in_chroma(&mut self) -> OffsetInChromaW<Swreg61Spec> {
        OffsetInChromaW::new(self, 20)
    }
}
#[doc = "input luminance information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_61::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_61::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg61Spec;
impl crate::RegisterSpec for Swreg61Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_61::R`](R) reader structure"]
impl crate::Readable for Swreg61Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg_61::W`](W) writer structure"]
impl crate::Writable for Swreg61Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_61 to value 0"]
impl crate::Resettable for Swreg61Spec {
    const RESET_VALUE: u32 = 0;
}

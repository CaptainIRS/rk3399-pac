#[doc = "Register `AWB_H_SIZE` reader"]
pub type R = crate::R<AwbHSizeSpec>;
#[doc = "Register `AWB_H_SIZE` writer"]
pub type W = crate::W<AwbHSizeSpec>;
#[doc = "Field `AWB_H_SIZE` reader - horizontal measurement window size in pixel\n\n"]
pub type AwbHSizeR = crate::FieldReader<u16>;
#[doc = "Field `AWB_H_SIZE` writer - horizontal measurement window size in pixel\n\n"]
pub type AwbHSizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - horizontal measurement window size in pixel\n\n"]
    #[inline(always)]
    pub fn awb_h_size(&self) -> AwbHSizeR {
        AwbHSizeR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - horizontal measurement window size in pixel\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn awb_h_size(&mut self) -> AwbHSizeW<AwbHSizeSpec> {
        AwbHSizeW::new(self, 0)
    }
}
#[doc = "Auto white balance horizontal window size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_h_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_h_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbHSizeSpec;
impl crate::RegisterSpec for AwbHSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_h_size::R`](R) reader structure"]
impl crate::Readable for AwbHSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_h_size::W`](W) writer structure"]
impl crate::Writable for AwbHSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_H_SIZE to value 0"]
impl crate::Resettable for AwbHSizeSpec {
    const RESET_VALUE: u32 = 0;
}

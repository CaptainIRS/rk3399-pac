#[doc = "Register `AWB_V_SIZE` reader"]
pub type R = crate::R<AwbVSizeSpec>;
#[doc = "Register `AWB_V_SIZE` writer"]
pub type W = crate::W<AwbVSizeSpec>;
#[doc = "Field `AWB_V_SIZE` reader - vertical measurement window size in lines"]
pub type AwbVSizeR = crate::FieldReader<u16>;
#[doc = "Field `AWB_V_SIZE` writer - vertical measurement window size in lines"]
pub type AwbVSizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - vertical measurement window size in lines"]
    #[inline(always)]
    pub fn awb_v_size(&self) -> AwbVSizeR {
        AwbVSizeR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - vertical measurement window size in lines"]
    #[inline(always)]
    #[must_use]
    pub fn awb_v_size(&mut self) -> AwbVSizeW<AwbVSizeSpec> {
        AwbVSizeW::new(self, 0)
    }
}
#[doc = "Auto white balance vertical window size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_v_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_v_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbVSizeSpec;
impl crate::RegisterSpec for AwbVSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_v_size::R`](R) reader structure"]
impl crate::Readable for AwbVSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_v_size::W`](W) writer structure"]
impl crate::Writable for AwbVSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_V_SIZE to value 0"]
impl crate::Resettable for AwbVSizeSpec {
    const RESET_VALUE: u32 = 0;
}

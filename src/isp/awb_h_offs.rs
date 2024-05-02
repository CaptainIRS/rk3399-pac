#[doc = "Register `AWB_H_OFFS` reader"]
pub type R = crate::R<AwbHOffsSpec>;
#[doc = "Register `AWB_H_OFFS` writer"]
pub type W = crate::W<AwbHOffsSpec>;
#[doc = "Field `AWB_H_OFFS` reader - horizontal window offset in pixel\n\n"]
pub type AwbHOffsR = crate::FieldReader<u16>;
#[doc = "Field `AWB_H_OFFS` writer - horizontal window offset in pixel\n\n"]
pub type AwbHOffsW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - horizontal window offset in pixel\n\n"]
    #[inline(always)]
    pub fn awb_h_offs(&self) -> AwbHOffsR {
        AwbHOffsR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - horizontal window offset in pixel\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn awb_h_offs(&mut self) -> AwbHOffsW<AwbHOffsSpec> {
        AwbHOffsW::new(self, 0)
    }
}
#[doc = "Auto white balance horizontal offset of measure window\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_h_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_h_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbHOffsSpec;
impl crate::RegisterSpec for AwbHOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_h_offs::R`](R) reader structure"]
impl crate::Readable for AwbHOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_h_offs::W`](W) writer structure"]
impl crate::Writable for AwbHOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_H_OFFS to value 0"]
impl crate::Resettable for AwbHOffsSpec {
    const RESET_VALUE: u32 = 0;
}

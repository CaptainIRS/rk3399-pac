#[doc = "Register `AWB_V_OFFS` reader"]
pub type R = crate::R<AwbVOffsSpec>;
#[doc = "Register `AWB_V_OFFS` writer"]
pub type W = crate::W<AwbVOffsSpec>;
#[doc = "Field `AWB_V_OFFS` reader - vertical window offset in lines\n\n"]
pub type AwbVOffsR = crate::FieldReader<u16>;
#[doc = "Field `AWB_V_OFFS` writer - vertical window offset in lines\n\n"]
pub type AwbVOffsW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - vertical window offset in lines\n\n"]
    #[inline(always)]
    pub fn awb_v_offs(&self) -> AwbVOffsR {
        AwbVOffsR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - vertical window offset in lines\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn awb_v_offs(&mut self) -> AwbVOffsW<AwbVOffsSpec> {
        AwbVOffsW::new(self, 0)
    }
}
#[doc = "Auto white balance vertical offset of measure window\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_v_offs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_v_offs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbVOffsSpec;
impl crate::RegisterSpec for AwbVOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_v_offs::R`](R) reader structure"]
impl crate::Readable for AwbVOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_v_offs::W`](W) writer structure"]
impl crate::Writable for AwbVOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_V_OFFS to value 0"]
impl crate::Resettable for AwbVOffsSpec {
    const RESET_VALUE: u32 = 0;
}

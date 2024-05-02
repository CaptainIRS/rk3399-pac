#[doc = "Register `AWB_REF` reader"]
pub type R = crate::R<AwbRefSpec>;
#[doc = "Register `AWB_REF` writer"]
pub type W = crate::W<AwbRefSpec>;
#[doc = "Field `AWB_REF_CB` reader - MAX_B\n\nreference Cb value for AWB regulation, target for AWB\n\nmaximum blue value, if RGB measurement mode is\n\nselected\n\n"]
pub type AwbRefCbR = crate::FieldReader;
#[doc = "Field `AWB_REF_CB` writer - MAX_B\n\nreference Cb value for AWB regulation, target for AWB\n\nmaximum blue value, if RGB measurement mode is\n\nselected\n\n"]
pub type AwbRefCbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AWB_REF_CR` reader - MAX_R\n\nreference Cr value for AWB regulation, target for AWB\n\nmaximum red value, if RGB measurement mode is\n\nselected"]
pub type AwbRefCrR = crate::FieldReader;
#[doc = "Field `AWB_REF_CR` writer - MAX_R\n\nreference Cr value for AWB regulation, target for AWB\n\nmaximum red value, if RGB measurement mode is\n\nselected"]
pub type AwbRefCrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - MAX_B\n\nreference Cb value for AWB regulation, target for AWB\n\nmaximum blue value, if RGB measurement mode is\n\nselected\n\n"]
    #[inline(always)]
    pub fn awb_ref_cb(&self) -> AwbRefCbR {
        AwbRefCbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MAX_R\n\nreference Cr value for AWB regulation, target for AWB\n\nmaximum red value, if RGB measurement mode is\n\nselected"]
    #[inline(always)]
    pub fn awb_ref_cr(&self) -> AwbRefCrR {
        AwbRefCrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MAX_B\n\nreference Cb value for AWB regulation, target for AWB\n\nmaximum blue value, if RGB measurement mode is\n\nselected\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn awb_ref_cb(&mut self) -> AwbRefCbW<AwbRefSpec> {
        AwbRefCbW::new(self, 0)
    }
    #[doc = "Bits 8:15 - MAX_R\n\nreference Cr value for AWB regulation, target for AWB\n\nmaximum red value, if RGB measurement mode is\n\nselected"]
    #[inline(always)]
    #[must_use]
    pub fn awb_ref_cr(&mut self) -> AwbRefCrW<AwbRefSpec> {
        AwbRefCrW::new(self, 8)
    }
}
#[doc = "Auto white balance reference Cb/Cr values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awb_ref::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awb_ref::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwbRefSpec;
impl crate::RegisterSpec for AwbRefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_ref::R`](R) reader structure"]
impl crate::Readable for AwbRefSpec {}
#[doc = "`write(|w| ..)` method takes [`awb_ref::W`](W) writer structure"]
impl crate::Writable for AwbRefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AWB_REF to value 0x8080"]
impl crate::Resettable for AwbRefSpec {
    const RESET_VALUE: u32 = 0x8080;
}

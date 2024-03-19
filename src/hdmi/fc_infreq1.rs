#[doc = "Register `FC_INFREQ1` reader"]
pub type R = crate::R<FcInfreq1Spec>;
#[doc = "Register `FC_INFREQ1` writer"]
pub type W = crate::W<FcInfreq1Spec>;
#[doc = "Field `INFREQ` reader - Video refresh rate in Hz*1E3 format. This register\n\nis provided for debug and informative purposes.\n\nThe Hdmi_tx does not write any data to this\n\nregister; the data written by software is not used\n\nby the Hdmi_tx."]
pub type InfreqR = crate::FieldReader;
#[doc = "Field `INFREQ` writer - Video refresh rate in Hz*1E3 format. This register\n\nis provided for debug and informative purposes.\n\nThe Hdmi_tx does not write any data to this\n\nregister; the data written by software is not used\n\nby the Hdmi_tx."]
pub type InfreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Video refresh rate in Hz*1E3 format. This register\n\nis provided for debug and informative purposes.\n\nThe Hdmi_tx does not write any data to this\n\nregister; the data written by software is not used\n\nby the Hdmi_tx."]
    #[inline(always)]
    pub fn infreq(&self) -> InfreqR {
        InfreqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Video refresh rate in Hz*1E3 format. This register\n\nis provided for debug and informative purposes.\n\nThe Hdmi_tx does not write any data to this\n\nregister; the data written by software is not used\n\nby the Hdmi_tx."]
    #[inline(always)]
    #[must_use]
    pub fn infreq(&mut self) -> InfreqW<FcInfreq1Spec> {
        InfreqW::new(self, 0)
    }
}
#[doc = "Frame Composer Input Video Refresh Rate Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_infreq1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_infreq1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcInfreq1Spec;
impl crate::RegisterSpec for FcInfreq1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_infreq1::R`](R) reader structure"]
impl crate::Readable for FcInfreq1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_infreq1::W`](W) writer structure"]
impl crate::Writable for FcInfreq1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_INFREQ1 to value 0"]
impl crate::Resettable for FcInfreq1Spec {
    const RESET_VALUE: u8 = 0;
}

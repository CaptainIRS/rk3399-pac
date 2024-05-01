#[doc = "Register `SWREG121` reader"]
pub type R = crate::R<Swreg121Spec>;
#[doc = "Register `SWREG121` writer"]
pub type W = crate::W<Swreg121Spec>;
#[doc = "Field `MFR_REG1` reader - multi format reuse register1 except h264\n\nRV:\n\n\\[20:16\\]
: frame size length\n\nJPEG:\n\n\\[26:0\\]
: progressive JPEG\n\nMPEG2 :\n\n\\[12\\]
: enable for bilinear motion compensation\n\nVP7:\n\n\\[31:26\\]
: DCT stream partition index 1 of start bit\n\n\\[25:20\\]
: DCT stream partition index 2 of start bit\n\n\\[13\\]
: rominance motion vector resolution for VP7/8\n\n\\[12\\]
: enable for bilinear motion compensation\n\n\\[11:9\\]
: 0st count for DC prediction mach\n\n\\[8:6\\]
: 1st count for DC prediction mach"]
pub type MfrReg1R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG1` writer - multi format reuse register1 except h264\n\nRV:\n\n\\[20:16\\]
: frame size length\n\nJPEG:\n\n\\[26:0\\]
: progressive JPEG\n\nMPEG2 :\n\n\\[12\\]
: enable for bilinear motion compensation\n\nVP7:\n\n\\[31:26\\]
: DCT stream partition index 1 of start bit\n\n\\[25:20\\]
: DCT stream partition index 2 of start bit\n\n\\[13\\]
: rominance motion vector resolution for VP7/8\n\n\\[12\\]
: enable for bilinear motion compensation\n\n\\[11:9\\]
: 0st count for DC prediction mach\n\n\\[8:6\\]
: 1st count for DC prediction mach"]
pub type MfrReg1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register1 except h264\n\nRV:\n\n\\[20:16\\]
: frame size length\n\nJPEG:\n\n\\[26:0\\]
: progressive JPEG\n\nMPEG2 :\n\n\\[12\\]
: enable for bilinear motion compensation\n\nVP7:\n\n\\[31:26\\]
: DCT stream partition index 1 of start bit\n\n\\[25:20\\]
: DCT stream partition index 2 of start bit\n\n\\[13\\]
: rominance motion vector resolution for VP7/8\n\n\\[12\\]
: enable for bilinear motion compensation\n\n\\[11:9\\]
: 0st count for DC prediction mach\n\n\\[8:6\\]
: 1st count for DC prediction mach"]
    #[inline(always)]
    pub fn mfr_reg1(&self) -> MfrReg1R {
        MfrReg1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register1 except h264\n\nRV:\n\n\\[20:16\\]
: frame size length\n\nJPEG:\n\n\\[26:0\\]
: progressive JPEG\n\nMPEG2 :\n\n\\[12\\]
: enable for bilinear motion compensation\n\nVP7:\n\n\\[31:26\\]
: DCT stream partition index 1 of start bit\n\n\\[25:20\\]
: DCT stream partition index 2 of start bit\n\n\\[13\\]
: rominance motion vector resolution for VP7/8\n\n\\[12\\]
: enable for bilinear motion compensation\n\n\\[11:9\\]
: 0st count for DC prediction mach\n\n\\[8:6\\]
: 1st count for DC prediction mach"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg1(&mut self) -> MfrReg1W<Swreg121Spec> {
        MfrReg1W::new(self, 0)
    }
}
#[doc = "multi format reuse register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg121::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg121::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg121Spec;
impl crate::RegisterSpec for Swreg121Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg121::R`](R) reader structure"]
impl crate::Readable for Swreg121Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg121::W`](W) writer structure"]
impl crate::Writable for Swreg121Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG121 to value 0"]
impl crate::Resettable for Swreg121Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SWREG136` reader"]
pub type R = crate::R<Swreg136Spec>;
#[doc = "Register `SWREG136` writer"]
pub type W = crate::W<Swreg136Spec>;
#[doc = "Field `MFR_REG16` reader - multi format reuse register16 except h264\n\nVP6/VP7\n\n\\[31:2\\]
: golden reference pic start address(PIC_ID 4)\n\n\\[0\\]
: golden reference pic siggn bias(VP7)\n\nMPEG4/MPEG2:\n\n\\[19\\]
: alternalte scan flag\n\n\\[18:15\\]
: HRZ AXI's bit amount for representing FWD MV\n\n\\[14:11\\]
: VRZ AXI's bit amount for representing FWD MV\n\n\\[10:7\\]
: HRZ AXI's bit amount for representing BWD MV\n\n\\[6:3\\]
: VRZ AXI's bit amount for representing BWD MV\n\n\\[2\\]
: FWD MV Y resolution\n\n\\[1\\]
: the ctrl bit for rounding(MPEG4),BWD MV Y\n\nresolution(MPEG2)\n\n\\[0\\]
: pic type of previous anchor(MPEG4)\n\nJPEG:\n\n\\[30:24\\]
: code words of length 14\n\n\\[23:16\\]
: code words of length 13\n\n\\[15:8\\]
: code words of length 12\n\n\\[7:0\\]
: code words of length 11"]
pub type MfrReg16R = crate::FieldReader<u32>;
#[doc = "Field `MFR_REG16` writer - multi format reuse register16 except h264\n\nVP6/VP7\n\n\\[31:2\\]
: golden reference pic start address(PIC_ID 4)\n\n\\[0\\]
: golden reference pic siggn bias(VP7)\n\nMPEG4/MPEG2:\n\n\\[19\\]
: alternalte scan flag\n\n\\[18:15\\]
: HRZ AXI's bit amount for representing FWD MV\n\n\\[14:11\\]
: VRZ AXI's bit amount for representing FWD MV\n\n\\[10:7\\]
: HRZ AXI's bit amount for representing BWD MV\n\n\\[6:3\\]
: VRZ AXI's bit amount for representing BWD MV\n\n\\[2\\]
: FWD MV Y resolution\n\n\\[1\\]
: the ctrl bit for rounding(MPEG4),BWD MV Y\n\nresolution(MPEG2)\n\n\\[0\\]
: pic type of previous anchor(MPEG4)\n\nJPEG:\n\n\\[30:24\\]
: code words of length 14\n\n\\[23:16\\]
: code words of length 13\n\n\\[15:8\\]
: code words of length 12\n\n\\[7:0\\]
: code words of length 11"]
pub type MfrReg16W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - multi format reuse register16 except h264\n\nVP6/VP7\n\n\\[31:2\\]
: golden reference pic start address(PIC_ID 4)\n\n\\[0\\]
: golden reference pic siggn bias(VP7)\n\nMPEG4/MPEG2:\n\n\\[19\\]
: alternalte scan flag\n\n\\[18:15\\]
: HRZ AXI's bit amount for representing FWD MV\n\n\\[14:11\\]
: VRZ AXI's bit amount for representing FWD MV\n\n\\[10:7\\]
: HRZ AXI's bit amount for representing BWD MV\n\n\\[6:3\\]
: VRZ AXI's bit amount for representing BWD MV\n\n\\[2\\]
: FWD MV Y resolution\n\n\\[1\\]
: the ctrl bit for rounding(MPEG4),BWD MV Y\n\nresolution(MPEG2)\n\n\\[0\\]
: pic type of previous anchor(MPEG4)\n\nJPEG:\n\n\\[30:24\\]
: code words of length 14\n\n\\[23:16\\]
: code words of length 13\n\n\\[15:8\\]
: code words of length 12\n\n\\[7:0\\]
: code words of length 11"]
    #[inline(always)]
    pub fn mfr_reg16(&self) -> MfrReg16R {
        MfrReg16R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - multi format reuse register16 except h264\n\nVP6/VP7\n\n\\[31:2\\]
: golden reference pic start address(PIC_ID 4)\n\n\\[0\\]
: golden reference pic siggn bias(VP7)\n\nMPEG4/MPEG2:\n\n\\[19\\]
: alternalte scan flag\n\n\\[18:15\\]
: HRZ AXI's bit amount for representing FWD MV\n\n\\[14:11\\]
: VRZ AXI's bit amount for representing FWD MV\n\n\\[10:7\\]
: HRZ AXI's bit amount for representing BWD MV\n\n\\[6:3\\]
: VRZ AXI's bit amount for representing BWD MV\n\n\\[2\\]
: FWD MV Y resolution\n\n\\[1\\]
: the ctrl bit for rounding(MPEG4),BWD MV Y\n\nresolution(MPEG2)\n\n\\[0\\]
: pic type of previous anchor(MPEG4)\n\nJPEG:\n\n\\[30:24\\]
: code words of length 14\n\n\\[23:16\\]
: code words of length 13\n\n\\[15:8\\]
: code words of length 12\n\n\\[7:0\\]
: code words of length 11"]
    #[inline(always)]
    #[must_use]
    pub fn mfr_reg16(&mut self) -> MfrReg16W<Swreg136Spec> {
        MfrReg16W::new(self, 0)
    }
}
#[doc = "multi format reuse register16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg136::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg136::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg136Spec;
impl crate::RegisterSpec for Swreg136Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg136::R`](R) reader structure"]
impl crate::Readable for Swreg136Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg136::W`](W) writer structure"]
impl crate::Writable for Swreg136Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG136 to value 0"]
impl crate::Resettable for Swreg136Spec {
    const RESET_VALUE: u32 = 0;
}

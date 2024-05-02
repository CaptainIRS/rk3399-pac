#[doc = "Register `FILT_MODE` reader"]
pub type R = crate::R<FiltModeSpec>;
#[doc = "Register `FILT_MODE` writer"]
pub type W = crate::W<FiltModeSpec>;
#[doc = "Field `filt_enable` reader - 1 enable filter\n\n0 bypass filter Default"]
pub type FiltEnableR = crate::BitReader;
#[doc = "Field `filt_enable` writer - 1 enable filter\n\n0 bypass filter Default"]
pub type FiltEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `filt_mode` reader - 0 green filter static mode (active filter factor = FILT_FAC_MID)\n\n1 dynamic noise reduction/sharpen Default\n\n"]
pub type FiltModeR = crate::BitReader;
#[doc = "Field `filt_mode` writer - 0 green filter static mode (active filter factor = FILT_FAC_MID)\n\n1 dynamic noise reduction/sharpen Default\n\n"]
pub type FiltModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `filt_chr_v_mode` reader - Chroma filter vertical mode\n\nvertical chroma filter bypass\n\nvertical chroma filter 1 static \\[8 16 8\\]\n\nvertical chroma filter 2 static \\[10 12 10\\]\n\nvertical chroma filter 3 static \\[12 8 12\\]
Default"]
pub type FiltChrVModeR = crate::FieldReader;
#[doc = "Field `filt_chr_v_mode` writer - Chroma filter vertical mode\n\nvertical chroma filter bypass\n\nvertical chroma filter 1 static \\[8 16 8\\]\n\nvertical chroma filter 2 static \\[10 12 10\\]\n\nvertical chroma filter 3 static \\[12 8 12\\]
Default"]
pub type FiltChrVModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `filt_chr_h_mode` reader - Chroma filter horizontal mode 00 horizontal chroma filter bypass 01 horizontal chroma filter 1 static mask = \\[10 12 10\\]
10 horizontal chroma filter 2 (dynamic blur1) 11 horizontal chroma filter 3 (dynamic blur2) Default"]
pub type FiltChrHModeR = crate::FieldReader;
#[doc = "Field `filt_chr_h_mode` writer - Chroma filter horizontal mode 00 horizontal chroma filter bypass 01 horizontal chroma filter 1 static mask = \\[10 12 10\\]
10 horizontal chroma filter 2 (dynamic blur1) 11 horizontal chroma filter 3 (dynamic blur2) Default"]
pub type FiltChrHModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `stage1_select` reader - Green filter stage 1 select (range 0x0...0x8) 0x0\n\nmaximum blurring\n\n0x4 Default\n\n0x7 minimum blurring 0x8 filter stage1 bypass\n\nFor a detailed description refer to chapter 'ISP Filter\n\nProgramming' of user manual"]
pub type Stage1SelectR = crate::FieldReader;
#[doc = "Field `stage1_select` writer - Green filter stage 1 select (range 0x0...0x8) 0x0\n\nmaximum blurring\n\n0x4 Default\n\n0x7 minimum blurring 0x8 filter stage1 bypass\n\nFor a detailed description refer to chapter 'ISP Filter\n\nProgramming' of user manual"]
pub type Stage1SelectW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 1 enable filter\n\n0 bypass filter Default"]
    #[inline(always)]
    pub fn filt_enable(&self) -> FiltEnableR {
        FiltEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0 green filter static mode (active filter factor = FILT_FAC_MID)\n\n1 dynamic noise reduction/sharpen Default\n\n"]
    #[inline(always)]
    pub fn filt_mode(&self) -> FiltModeR {
        FiltModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Chroma filter vertical mode\n\nvertical chroma filter bypass\n\nvertical chroma filter 1 static \\[8 16 8\\]\n\nvertical chroma filter 2 static \\[10 12 10\\]\n\nvertical chroma filter 3 static \\[12 8 12\\]
Default"]
    #[inline(always)]
    pub fn filt_chr_v_mode(&self) -> FiltChrVModeR {
        FiltChrVModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Chroma filter horizontal mode 00 horizontal chroma filter bypass 01 horizontal chroma filter 1 static mask = \\[10 12 10\\]
10 horizontal chroma filter 2 (dynamic blur1) 11 horizontal chroma filter 3 (dynamic blur2) Default"]
    #[inline(always)]
    pub fn filt_chr_h_mode(&self) -> FiltChrHModeR {
        FiltChrHModeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Green filter stage 1 select (range 0x0...0x8) 0x0\n\nmaximum blurring\n\n0x4 Default\n\n0x7 minimum blurring 0x8 filter stage1 bypass\n\nFor a detailed description refer to chapter 'ISP Filter\n\nProgramming' of user manual"]
    #[inline(always)]
    pub fn stage1_select(&self) -> Stage1SelectR {
        Stage1SelectR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1 enable filter\n\n0 bypass filter Default"]
    #[inline(always)]
    #[must_use]
    pub fn filt_enable(&mut self) -> FiltEnableW<FiltModeSpec> {
        FiltEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 0 green filter static mode (active filter factor = FILT_FAC_MID)\n\n1 dynamic noise reduction/sharpen Default\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn filt_mode(&mut self) -> FiltModeW<FiltModeSpec> {
        FiltModeW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Chroma filter vertical mode\n\nvertical chroma filter bypass\n\nvertical chroma filter 1 static \\[8 16 8\\]\n\nvertical chroma filter 2 static \\[10 12 10\\]\n\nvertical chroma filter 3 static \\[12 8 12\\]
Default"]
    #[inline(always)]
    #[must_use]
    pub fn filt_chr_v_mode(&mut self) -> FiltChrVModeW<FiltModeSpec> {
        FiltChrVModeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Chroma filter horizontal mode 00 horizontal chroma filter bypass 01 horizontal chroma filter 1 static mask = \\[10 12 10\\]
10 horizontal chroma filter 2 (dynamic blur1) 11 horizontal chroma filter 3 (dynamic blur2) Default"]
    #[inline(always)]
    #[must_use]
    pub fn filt_chr_h_mode(&mut self) -> FiltChrHModeW<FiltModeSpec> {
        FiltChrHModeW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Green filter stage 1 select (range 0x0...0x8) 0x0\n\nmaximum blurring\n\n0x4 Default\n\n0x7 minimum blurring 0x8 filter stage1 bypass\n\nFor a detailed description refer to chapter 'ISP Filter\n\nProgramming' of user manual"]
    #[inline(always)]
    #[must_use]
    pub fn stage1_select(&mut self) -> Stage1SelectW<FiltModeSpec> {
        Stage1SelectW::new(self, 8)
    }
}
#[doc = "mode control register for the filter block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filt_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filt_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiltModeSpec;
impl crate::RegisterSpec for FiltModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filt_mode::R`](R) reader structure"]
impl crate::Readable for FiltModeSpec {}
#[doc = "`write(|w| ..)` method takes [`filt_mode::W`](W) writer structure"]
impl crate::Writable for FiltModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILT_MODE to value 0x04f2"]
impl crate::Resettable for FiltModeSpec {
    const RESET_VALUE: u32 = 0x04f2;
}

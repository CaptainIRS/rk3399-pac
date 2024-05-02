#[doc = "Register `BLS_CTRL` reader"]
pub type R = crate::R<BlsCtrlSpec>;
#[doc = "Register `BLS_CTRL` writer"]
pub type W = crate::W<BlsCtrlSpec>;
#[doc = "Field `BLS_ENABLE` reader - 1: black level subtraction is\n\nenabled 0: bypass the black\n\nlevel processing\n\n"]
pub type BlsEnableR = crate::BitReader;
#[doc = "Field `BLS_ENABLE` writer - 1: black level subtraction is\n\nenabled 0: bypass the black\n\nlevel processing\n\n"]
pub type BlsEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `is` reader - measured\n\n2: only window 2 is\n\nmeasured 3: both windows 1 BLS_MODE 1: subtract measured\n\nvalues 0: subtract fixed\n\nvalues"]
pub type IsR = crate::BitReader;
#[doc = "Field `is` writer - measured\n\n2: only window 2 is\n\nmeasured 3: both windows 1 BLS_MODE 1: subtract measured\n\nvalues 0: subtract fixed\n\nvalues"]
pub type IsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINDOW_ENABLE` reader - 0: no measuring is\n\nperformed 1: only window"]
pub type WindowEnableR = crate::FieldReader;
#[doc = "Field `WINDOW_ENABLE` writer - 0: no measuring is\n\nperformed 1: only window"]
pub type WindowEnableW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 1: black level subtraction is\n\nenabled 0: bypass the black\n\nlevel processing\n\n"]
    #[inline(always)]
    pub fn bls_enable(&self) -> BlsEnableR {
        BlsEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - measured\n\n2: only window 2 is\n\nmeasured 3: both windows 1 BLS_MODE 1: subtract measured\n\nvalues 0: subtract fixed\n\nvalues"]
    #[inline(always)]
    pub fn is(&self) -> IsR {
        IsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 0: no measuring is\n\nperformed 1: only window"]
    #[inline(always)]
    pub fn window_enable(&self) -> WindowEnableR {
        WindowEnableR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1: black level subtraction is\n\nenabled 0: bypass the black\n\nlevel processing\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn bls_enable(&mut self) -> BlsEnableW<BlsCtrlSpec> {
        BlsEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - measured\n\n2: only window 2 is\n\nmeasured 3: both windows 1 BLS_MODE 1: subtract measured\n\nvalues 0: subtract fixed\n\nvalues"]
    #[inline(always)]
    #[must_use]
    pub fn is(&mut self) -> IsW<BlsCtrlSpec> {
        IsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 0: no measuring is\n\nperformed 1: only window"]
    #[inline(always)]
    #[must_use]
    pub fn window_enable(&mut self) -> WindowEnableW<BlsCtrlSpec> {
        WindowEnableW::new(self, 2)
    }
}
#[doc = "global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bls_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bls_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlsCtrlSpec;
impl crate::RegisterSpec for BlsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bls_ctrl::R`](R) reader structure"]
impl crate::Readable for BlsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bls_ctrl::W`](W) writer structure"]
impl crate::Writable for BlsCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLS_CTRL to value 0"]
impl crate::Resettable for BlsCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

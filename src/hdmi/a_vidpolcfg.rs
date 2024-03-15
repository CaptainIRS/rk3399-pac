#[doc = "Register `A_VIDPOLCFG` reader"]
pub type R = crate::R<AVidpolcfgSpec>;
#[doc = "Register `A_VIDPOLCFG` writer"]
pub type W = crate::W<AVidpolcfgSpec>;
#[doc = "Field `SPARE_1` reader - Reserved as \"spare\" bit with no associated functionality."]
pub type Spare1R = crate::BitReader;
#[doc = "Field `SPARE_1` writer - Reserved as \"spare\" bit with no associated functionality."]
pub type Spare1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSYNCPOL` reader - Configuration of the video Horizontal synchronism polarity."]
pub type HsyncpolR = crate::BitReader;
#[doc = "Field `HSYNCPOL` writer - Configuration of the video Horizontal synchronism polarity."]
pub type HsyncpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE_2` reader - Reserved as \"spare\" bit with no associated functionality."]
pub type Spare2R = crate::BitReader;
#[doc = "Field `SPARE_2` writer - Reserved as \"spare\" bit with no associated functionality."]
pub type Spare2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNCPOL` reader - Configuration of the video Vertical synchronism polarity"]
pub type VsyncpolR = crate::BitReader;
#[doc = "Field `VSYNCPOL` writer - Configuration of the video Vertical synchronism polarity"]
pub type VsyncpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENPOL` reader - Configuration of the video data enable polarity"]
pub type DataenpolR = crate::BitReader;
#[doc = "Field `DATAENPOL` writer - Configuration of the video data enable polarity"]
pub type DataenpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNENCRYPTCONF` reader - Configuration of the color sent when sending unencrypted video data"]
pub type UnencryptconfR = crate::FieldReader;
#[doc = "Field `UNENCRYPTCONF` writer - Configuration of the color sent when sending unencrypted video data"]
pub type UnencryptconfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Reserved as \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn spare_1(&self) -> Spare1R {
        Spare1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration of the video Horizontal synchronism polarity."]
    #[inline(always)]
    pub fn hsyncpol(&self) -> HsyncpolR {
        HsyncpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved as \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn spare_2(&self) -> Spare2R {
        Spare2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configuration of the video Vertical synchronism polarity"]
    #[inline(always)]
    pub fn vsyncpol(&self) -> VsyncpolR {
        VsyncpolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration of the video data enable polarity"]
    #[inline(always)]
    pub fn dataenpol(&self) -> DataenpolR {
        DataenpolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Configuration of the color sent when sending unencrypted video data"]
    #[inline(always)]
    pub fn unencryptconf(&self) -> UnencryptconfR {
        UnencryptconfR::new((self.bits >> 5) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved as \"spare\" bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare_1(&mut self) -> Spare1W<AVidpolcfgSpec> {
        Spare1W::new(self, 0)
    }
    #[doc = "Bit 1 - Configuration of the video Horizontal synchronism polarity."]
    #[inline(always)]
    #[must_use]
    pub fn hsyncpol(&mut self) -> HsyncpolW<AVidpolcfgSpec> {
        HsyncpolW::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved as \"spare\" bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn spare_2(&mut self) -> Spare2W<AVidpolcfgSpec> {
        Spare2W::new(self, 2)
    }
    #[doc = "Bit 3 - Configuration of the video Vertical synchronism polarity"]
    #[inline(always)]
    #[must_use]
    pub fn vsyncpol(&mut self) -> VsyncpolW<AVidpolcfgSpec> {
        VsyncpolW::new(self, 3)
    }
    #[doc = "Bit 4 - Configuration of the video data enable polarity"]
    #[inline(always)]
    #[must_use]
    pub fn dataenpol(&mut self) -> DataenpolW<AVidpolcfgSpec> {
        DataenpolW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Configuration of the color sent when sending unencrypted video data"]
    #[inline(always)]
    #[must_use]
    pub fn unencryptconf(&mut self) -> UnencryptconfW<AVidpolcfgSpec> {
        UnencryptconfW::new(self, 5)
    }
}
#[doc = "Reserved as \"spare\" bit with no associated functionality.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`a_vidpolcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`a_vidpolcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AVidpolcfgSpec;
impl crate::RegisterSpec for AVidpolcfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a_vidpolcfg::R`](R) reader structure"]
impl crate::Readable for AVidpolcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`a_vidpolcfg::W`](W) writer structure"]
impl crate::Writable for AVidpolcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets A_VIDPOLCFG to value 0"]
impl crate::Resettable for AVidpolcfgSpec {
    const RESET_VALUE: u8 = 0;
}

#[doc = "Register `MSCH_DeviceConf` reader"]
pub type R = crate::R<MschDeviceConfSpec>;
#[doc = "Register `MSCH_DeviceConf` writer"]
pub type W = crate::W<MschDeviceConfSpec>;
#[doc = "Field `RANK0` reader - Rank0 selector of predefined ddrConf configuration"]
pub type Rank0R = crate::FieldReader;
#[doc = "Field `RANK0` writer - Rank0 selector of predefined ddrConf configuration"]
pub type Rank0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RANK1` reader - Rank1 selector of predefined ddrConf configuration"]
pub type Rank1R = crate::FieldReader;
#[doc = "Field `RANK1` writer - Rank1 selector of predefined ddrConf configuration"]
pub type Rank1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Rank0 selector of predefined ddrConf configuration"]
    #[inline(always)]
    pub fn rank0(&self) -> Rank0R {
        Rank0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Rank1 selector of predefined ddrConf configuration"]
    #[inline(always)]
    pub fn rank1(&self) -> Rank1R {
        Rank1R::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Rank0 selector of predefined ddrConf configuration"]
    #[inline(always)]
    #[must_use]
    pub fn rank0(&mut self) -> Rank0W<MschDeviceConfSpec> {
        Rank0W::new(self, 0)
    }
    #[doc = "Bits 6:11 - Rank1 selector of predefined ddrConf configuration"]
    #[inline(always)]
    #[must_use]
    pub fn rank1(&mut self) -> Rank1W<MschDeviceConfSpec> {
        Rank1W::new(self, 6)
    }
}
#[doc = "ddr configuration pointers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_device_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_device_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MschDeviceConfSpec;
impl crate::RegisterSpec for MschDeviceConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msch_device_conf::R`](R) reader structure"]
impl crate::Readable for MschDeviceConfSpec {}
#[doc = "`write(|w| ..)` method takes [`msch_device_conf::W`](W) writer structure"]
impl crate::Writable for MschDeviceConfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSCH_DeviceConf to value 0"]
impl crate::Resettable for MschDeviceConfSpec {
    const RESET_VALUE: u32 = 0;
}

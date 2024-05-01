#[doc = "Register `CONFIG_DONE` reader"]
pub type R = crate::R<ConfigDoneSpec>;
#[doc = "Register `CONFIG_DONE` writer"]
pub type W = crate::W<ConfigDoneSpec>;
#[doc = "Field `CONFIG_DONE` reader - configuration done\n\nWait for frame start to update raw register configuration to really\n\nused registers."]
pub type ConfigDoneR = crate::BitReader;
#[doc = "Field `CONFIG_DONE` writer - configuration done\n\nWait for frame start to update raw register configuration to really\n\nused registers."]
pub type ConfigDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - configuration done\n\nWait for frame start to update raw register configuration to really\n\nused registers."]
    #[inline(always)]
    pub fn config_done(&self) -> ConfigDoneR {
        ConfigDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configuration done\n\nWait for frame start to update raw register configuration to really\n\nused registers."]
    #[inline(always)]
    #[must_use]
    pub fn config_done(&mut self) -> ConfigDoneW<ConfigDoneSpec> {
        ConfigDoneW::new(self, 0)
    }
}
#[doc = "configuration done\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_done::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_done::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigDoneSpec;
impl crate::RegisterSpec for ConfigDoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_done::R`](R) reader structure"]
impl crate::Readable for ConfigDoneSpec {}
#[doc = "`write(|w| ..)` method takes [`config_done::W`](W) writer structure"]
impl crate::Writable for ConfigDoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_DONE to value 0"]
impl crate::Resettable for ConfigDoneSpec {
    const RESET_VALUE: u32 = 0;
}

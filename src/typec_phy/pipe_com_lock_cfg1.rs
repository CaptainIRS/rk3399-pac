#[doc = "Register `PIPE_COM_LOCK_CFG1` reader"]
pub type R = crate::R<PipeComLockCfg1Spec>;
#[doc = "Register `PIPE_COM_LOCK_CFG1` writer"]
pub type W = crate::W<PipeComLockCfg1Spec>;
#[doc = "Field `FIELD1` reader - comma full lock count: The number of COMMA symbols that need \n\nto be seen in the same bit position for the comma alignment state \n\nmachine to lock. The field is used for initial reset lock."]
pub type Field1R = crate::FieldReader<u16>;
#[doc = "Field `FIELD1` writer - comma full lock count: The number of COMMA symbols that need \n\nto be seen in the same bit position for the comma alignment state \n\nmachine to lock. The field is used for initial reset lock."]
pub type Field1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FIELD0` reader - Symbol unlock count: The number of COMMA symbols that need \n\nto be seen in the wrong bit position before the comma alignment \n\nstate machine will transition to RESYNC or LOS state"]
pub type Field0R = crate::FieldReader;
#[doc = "Field `FIELD0` writer - Symbol unlock count: The number of COMMA symbols that need \n\nto be seen in the wrong bit position before the comma alignment \n\nstate machine will transition to RESYNC or LOS state"]
pub type Field0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - comma full lock count: The number of COMMA symbols that need \n\nto be seen in the same bit position for the comma alignment state \n\nmachine to lock. The field is used for initial reset lock."]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(self.bits & 0x0fff)
    }
    #[doc = "Bits 12:15 - Symbol unlock count: The number of COMMA symbols that need \n\nto be seen in the wrong bit position before the comma alignment \n\nstate machine will transition to RESYNC or LOS state"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - comma full lock count: The number of COMMA symbols that need \n\nto be seen in the same bit position for the comma alignment state \n\nmachine to lock. The field is used for initial reset lock."]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<PipeComLockCfg1Spec> {
        Field1W::new(self, 0)
    }
    #[doc = "Bits 12:15 - Symbol unlock count: The number of COMMA symbols that need \n\nto be seen in the wrong bit position before the comma alignment \n\nstate machine will transition to RESYNC or LOS state"]
    #[inline(always)]
    #[must_use]
    pub fn field0(&mut self) -> Field0W<PipeComLockCfg1Spec> {
        Field0W::new(self, 12)
    }
}
#[doc = "PIPE comma lock configuration1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipe_com_lock_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipe_com_lock_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PipeComLockCfg1Spec;
impl crate::RegisterSpec for PipeComLockCfg1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipe_com_lock_cfg1::R`](R) reader structure"]
impl crate::Readable for PipeComLockCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pipe_com_lock_cfg1::W`](W) writer structure"]
impl crate::Writable for PipeComLockCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PIPE_COM_LOCK_CFG1 to value 0x4400"]
impl crate::Resettable for PipeComLockCfg1Spec {
    const RESET_VALUE: u16 = 0x4400;
}

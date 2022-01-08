#[doc = "Register `CE_TPR` reader"]
pub struct R(crate::R<CE_TPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_TPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_TPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_TPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CE_TPR` writer"]
pub struct W(crate::W<CE_TPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_TPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CE_TPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_TPR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Throughput Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_tpr](index.html) module"]
pub struct CE_TPR_SPEC;
impl crate::RegisterSpec for CE_TPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_tpr::R](R) reader structure"]
impl crate::Readable for CE_TPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_tpr::W](W) writer structure"]
impl crate::Writable for CE_TPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CE_TPR to value 0"]
impl crate::Resettable for CE_TPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
